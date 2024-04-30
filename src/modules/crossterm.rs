use std::{
    io::{self, Stdout, Write},
    time::Duration,
};

use anyhow::{Error, Result};
use crossterm::{
    cursor::{self, MoveToColumn, MoveToRow},
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyEventState, KeyModifiers, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags
    },
    queue,
    style::{self, Color, Print, SetBackgroundColor, Stylize},
    terminal::{self, supports_keyboard_enhancement},
    ExecutableCommand, QueueableCommand,
};
use log::{error, log};

use super::ui::{Rect, UI};

pub struct Crossterm {
    pub screen: Rect,
    pub stdout: Stdout,
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
