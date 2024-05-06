use std::{
    borrow::BorrowMut,
    io::{self, Stdout, Write},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    cursor::{MoveTo, MoveToColumn},
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    queue,
    style::{
        Attributes, Color, Print, PrintStyledContent, SetBackgroundColor, StyledContent, Stylize,
    },
    terminal::{self, supports_keyboard_enhancement},
    ExecutableCommand, QueueableCommand,
};
use log::error;

use super::ui::{Overflow, Rect, Render, UI};

#[derive(Clone)]
struct Cell {
    attribute: Attributes,
    char: String,
    fg: Color,
    bg: Color,
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            attribute: Attributes::default(),
            char: " ".to_owned(),
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

struct Buffer {
    data: Vec<Cell>,
    width: usize,
    height: usize,
}
impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Cell::default(); width * height],
            width,
            height,
        }
    }

    fn get(&mut self, x: usize, y: usize) -> &mut Cell {
        self.data[y * self.width + x].borrow_mut()
    }
}

pub struct Crossterm {
    pub screen: Rect,
    pub stdout: Stdout,
    buffer: Buffer,
}

impl UI for Crossterm {
    fn init() -> Result<Self> {
        terminal::enable_raw_mode()?;
        let size = terminal::size()?;
        let mut ct = Self {
            stdout: io::stdout(),
            screen: Rect {
                x: 0,
                y: 0,
                width: size.0 as u32,
                height: size.1 as u32,
            },
            buffer: Buffer::new(size.0.into(), size.1.into()),
        };
        ct.stdout.execute(EnableMouseCapture)?;
        if supports_keyboard_enhancement()? {
            ct.stdout
                .execute(PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all()))?;
        }

        ct.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(ct)
    }
    fn read_event(&mut self) -> anyhow::Result<crossterm::event::Event> {
        match read() {
            Ok(event) => {
                if let Event::Resize(cols, rows) = event {
                    self.screen.width = cols.into();
                    self.screen.height = rows.into();
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
        self.stdout
            .queue(DisableMouseCapture)?
            .queue(PopKeyboardEnhancementFlags)?
            .queue(SetBackgroundColor(Color::Reset))?
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(MoveToColumn(0))?;
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
                let cell = self.buffer.get(x as usize, y as usize);
                cell.char = " ".to_string();
            }
        }

        }

        let mut x = 0;
        let mut y = 0;
        
        match overflow {
            Overflow::Char => {
                for (idx, char) in text.char_indices() {
                    if idx % rect.width as usize == 0 && idx != 0 || char == '\n'{
                        y += 1;
                        x = 0;
                    }
                    if y >= rect.height {
                        break;
                    }
                    if char =='\n'{
                        continue;
                    }
                    let cell = self.buffer.get(x + rect.x as usize, (y + rect.y) as usize);
                    cell.char = char.to_string();
                    x += 1;
                }
            }
            Overflow::Word => {
                todo!()
            }
        }
    }
    // for x in rect.x..(rect.x + rect.width) {
    //     for y in rect.y..(rect.y + rect.height) {
    //     }
    //     }

    fn set_background(&mut self, rect: Rect, color: Color) {
        for x in rect.x..(rect.x + rect.width) {
            for y in rect.y..(rect.y + rect.height) {
                let cell = self.buffer.get(x as usize, y as usize);
                cell.bg = color;
            }
        }
    }

    fn render_frame(&mut self) {
        for (x, y, cell) in self.buffer.data.iter().enumerate().map(|(idx, cell)| {
            (
                idx % self.buffer.width,
                (idx as f32 / self.buffer.width as f32).floor() as usize,
                cell,
            )
        }) {
            let _ = queue!(
                self.stdout,
                MoveTo(x as u16, y as u16),
                PrintStyledContent(
                    cell.char.clone().with(cell.fg).on(cell.bg) // .attribute(cell.attribute)
                )
            );
        }
    }
}
