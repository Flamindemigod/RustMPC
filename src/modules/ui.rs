use anyhow::Result;
use crossterm::{event::{Event, KeyEvent}, style::Color};

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

// UI Trait contains all mechanisms that platform needs to implement
pub trait UI {
    fn init() -> Result<Self>
    where
        Self: Sized;
    // Key/Mouse Events
    fn is_event_ready(&self) -> bool;

    //Read Event
    fn read_event(&mut self) -> Result<Event>;

    fn destroy(&mut self) -> Result<()>;
}

pub trait Render {
    fn set_background(&mut self, rect: Rect, color: Color);
    fn set_text(&mut self, rect: Rect, text: &str, overflow: Overflow);
    fn render_frame(&mut self) -> Result<()>;
}

pub enum Overflow{
    Char,
    Word,
}
