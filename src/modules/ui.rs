use anyhow::Result;
use crossterm::event::{Event, KeyEvent};

// UI Trait contains all mechanisms that platform needs to implement
pub trait UI {
    fn init(&self);
    // Key/Mouse Events
    fn is_event_ready(&self) -> bool;

    //Read Event
    fn read_event(&self) -> Result<Event>;
}

pub trait Render {
    fn set_background(rect: Event);
}
