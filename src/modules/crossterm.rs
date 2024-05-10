use std::{
    borrow::BorrowMut,
    io::{self, Stdout, Write},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    execute, queue,
    style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::{self, supports_keyboard_enhancement, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::error;

use super::ui::{Overflow, Rect, Render, UI};

#[derive(Clone)]
struct Patch {
    x: u32,
    y: u32,
    data: Cell,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cell {
    attribute: Option<Attribute>,
    char: String,
    fg: Color,
    bg: Color,
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            attribute: None,
            char: " ".to_owned(),
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

#[derive(Clone)]
struct Buffer {
    data: Vec<Cell>,
    screen: Rect,
}
impl Buffer {
    fn new(rect: Rect) -> Self {
        Self {
            data: vec![Cell::default(); (rect.height * rect.width) as usize],
            screen: rect,
        }
    }

    fn get(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let idx = y * self.screen.width as usize + x;
        if idx < self.data.len() {
            Some(self.data[idx].borrow_mut())
        } else {
            None
        }
    }

    fn diff(&self, other: &Self) -> Vec<Patch> {
        if self.data.len() != other.data.len() {
            self.data
                .iter()
                .enumerate()
                .map(|(idx, a)| Patch {
                    data: a.clone(),
                    x: idx as u32 % self.screen.width,
                    y: idx as u32 / self.screen.width,
                })
                .collect::<Vec<_>>()
        } else {
            self.data
                .iter()
                .zip(&other.data)
                .enumerate()
                .filter_map(|(idx, (a, b))| {
                    if a.ne(b) {
                        Some(Patch {
                            data: a.clone(),
                            x: idx as u32 % self.screen.width,
                            y: idx as u32 / self.screen.width,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
    }
}

pub struct Crossterm {
    pub screen: Rect,
    pub stdout: Stdout,
    prev_buffer: Buffer,
    buffer: Buffer,
}

impl UI for Crossterm {
    fn init() -> Result<Self> {
        terminal::enable_raw_mode()?;
        let size = terminal::size()?;
        let rect = Rect {
            x: 0,
            y: 0,
            width: size.0 as u32,
            height: size.1 as u32,
        };
        let mut ct = Self {
            stdout: io::stdout(),
            screen: rect,
            buffer: Buffer::new(rect),
            prev_buffer: Buffer::new(rect),
        };
        execute!(ct.stdout, EnableMouseCapture, EnterAlternateScreen, Hide)?;
        if supports_keyboard_enhancement()? {
            ct.stdout
                .execute(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all()))?;
        }
        Ok(ct)
    }
    fn read_event(&mut self) -> anyhow::Result<crossterm::event::Event> {
        match read() {
            Ok(event) => {
                if let Event::Resize(cols, rows) = event {
                    self.screen.width = cols.into();
                    self.screen.height = rows.into();
                    self.buffer = Buffer::new(self.screen);
                }
                Ok(event)
            }
            Err(err) => {
                error!("Failed to read Event: {err}");
                Err(err.into())
            }
        }
    }
    fn is_event_ready(&self) -> bool {
        poll(Duration::from_millis(250)).unwrap_or(false)
    }
    fn destroy(&mut self) -> Result<()> {
        queue!(
            self.stdout,
            DisableMouseCapture,
            PopKeyboardEnhancementFlags,
            LeaveAlternateScreen
        )?;
        self.stdout.flush()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Render for Crossterm {
    fn set_text(&mut self, rect: Rect, text: &str, overflow: Overflow) {
        {
            for x in rect.x..(rect.x + rect.width) {
                for y in rect.y..(rect.y + rect.height) {
                    if let Some(cell) = self.buffer.get(x as usize, y as usize) {
                        cell.char = " ".to_string()
                    };
                }
            }
        }

        let mut x = 0;
        let mut y = 0;

        match overflow {
            Overflow::Char => {
                for (idx, char) in text.char_indices() {
                    if idx % rect.width as usize == 0 && idx != 0 || char == '\n' {
                        y += 1;
                        x = 0;
                    }
                    if y >= rect.height {
                        break;
                    }
                    if char == '\n' {
                        continue;
                    }
                    if let Some(cell) = self.buffer.get(x + rect.x as usize, (y + rect.y) as usize)
                    {
                        cell.char = char.to_string();

                        x += 1;
                    }
                }
            }
            Overflow::Word => {
                todo!()
            }
        }
    }

    fn set_background(&mut self, rect: Rect, color: Color) {
        for x in rect.x..(rect.x + rect.width) {
            for y in rect.y..(rect.y + rect.height) {
                if let Some(cell) = self.buffer.get(x as usize, y as usize) {
                    cell.bg = color
                };
            }
        }
    }
    fn set_foreground(&mut self, rect: Rect, color: Color) {
        for x in rect.x..(rect.x + rect.width) {
            for y in rect.y..(rect.y + rect.height) {
                if let Some(cell) = self.buffer.get(x as usize, y as usize) {
                    cell.fg = color
                };
            }
        }
    }
    fn set_attributes(&mut self, rect: Rect, attr: Attribute) {
        for x in rect.x..(rect.x + rect.width) {
            for y in rect.y..(rect.y + rect.height) {
                if let Some(cell) = self.buffer.get(x as usize, y as usize) {
                    cell.attribute = Some(attr);
                };
            }
        }
    }
    fn render_frame(&mut self) -> Result<()> {
        let patches = self.buffer.diff(&self.prev_buffer);
        for patch in &patches {
            let mut p = patch
                .data
                .char
                .clone()
                .with(patch.data.fg)
                .on(patch.data.bg);
            if let Some(attr) = patch.data.attribute {
                p = p.attribute(attr);
            };
            queue!(
                self.stdout,
                MoveTo(patch.x as u16, patch.y as u16),
                PrintStyledContent(p)
            )?;
        }
        if !patches.is_empty() {
            self.stdout.flush()?;
            self.prev_buffer = self.buffer.clone();
        }
        Ok(())
    }
}
