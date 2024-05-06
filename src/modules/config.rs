use std::{
    fs::{self, File},
    io::Write,
    net::{SocketAddr, ToSocketAddrs},
    ops::{Deref, DerefMut},
};

use anyhow::{Error, Ok, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use dirs::config_dir;
use log::{error, info};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Serialize, Deserialize)]
pub struct Mpd {
    addr: Either<SocketAddr, Box<str>>,
    password: Option<Box<str>>,
}

impl Default for Mpd {
    fn default() -> Self {
        Self {
            addr: Either::Left(SocketAddr::new([127, 0, 0, 1].into(), 6600)),
            password: None,
        }
    }
}

impl Mpd {
    pub fn get_addr(&self) -> Result<SocketAddr> {
        match &self.addr {
            //IpAddr
            Either::Left(addr) => Ok(*addr),
            //String
            Either::Right(addr) => Ok(addr.to_socket_addrs()?.next().unwrap()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EventWrapper(Vec<Event>);

impl Deref for EventWrapper {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EventWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl EventWrapper {
    pub fn matches(&self, case: &Event) -> bool {
        self.iter().any(|v| v == case)
    }
}
impl From<Vec<Event>> for EventWrapper {
    fn from(value: Vec<Event>) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Keybinds {
    //UI Keybinds
    pub up: EventWrapper,
    pub down: EventWrapper,

    //Player Keybinds
    pub play_pause: EventWrapper,
    pub stop: EventWrapper,
    pub clear_queue: EventWrapper,
    pub add_to_queue: EventWrapper,
    pub next: EventWrapper,
    pub prev: EventWrapper,
    pub vol_up: EventWrapper,
    pub vol_down: EventWrapper,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            up: vec![Event::Key(KeyEvent::new(
                KeyCode::Char('j'),
                KeyModifiers::NONE,
            ))]
            .into(),
            down: vec![Event::Key(KeyEvent::new(
                KeyCode::Char('k'),
                KeyModifiers::NONE,
            ))]
            .into(),
            play_pause: vec![
                Event::Key(KeyEvent::new(
                    KeyCode::Media(crossterm::event::MediaKeyCode::Play),
                    KeyModifiers::NONE,
                )),
                Event::Key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)),
            ]
            .into(),
            stop: vec![
                Event::Key(KeyEvent::new(
                    KeyCode::Media(crossterm::event::MediaKeyCode::Stop),
                    KeyModifiers::NONE,
                )),
                Event::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)),
            ]
            .into(),
            clear_queue: vec![Event::Key(KeyEvent::new(
                KeyCode::Char('d'),
                KeyModifiers::NONE,
            ))]
            .into(),
            add_to_queue: vec![Event::Key(KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::NONE,
            ))]
            .into(),
            next: vec![
                Event::Key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE)),
                Event::Key(KeyEvent::new(
                    KeyCode::Media(crossterm::event::MediaKeyCode::TrackNext),
                    KeyModifiers::NONE,
                )),
            ]
            .into(),
            prev: vec![
                Event::Key(KeyEvent::new(KeyCode::Char('N'), KeyModifiers::NONE)),
                Event::Key(KeyEvent::new(
                    KeyCode::Media(crossterm::event::MediaKeyCode::TrackPrevious),
                    KeyModifiers::NONE,
                )),
            ]
            .into(),
            vol_up: vec![
                Event::Key(KeyEvent::new(KeyCode::Char('+'), KeyModifiers::NONE)),
                Event::Key(KeyEvent::new(
                    KeyCode::Media(crossterm::event::MediaKeyCode::RaiseVolume),
                    KeyModifiers::NONE,
                )),
            ]
            .into(),
            vol_down: vec![
                Event::Key(KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE)),
                Event::Key(KeyEvent::new(
                    KeyCode::Media(crossterm::event::MediaKeyCode::LowerVolume),
                    KeyModifiers::NONE,
                )),
            ]
            .into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub mpd: Mpd,
    pub keybinds: Keybinds,
}

impl Config {
    pub fn generate_config(&self) -> Result<()> {
        let home = config_dir();
        if let Some(p) = home {
            let conf_path = p.join(env!("CARGO_PKG_NAME")).join("config.yaml");
            if conf_path.exists() {
                info!("Config exists. Skipping Generation");
                return Ok(());
            }
            fs::create_dir_all(
                conf_path
                    .parent()
                    .expect("Failed to get Config Parent path"),
            )?;

            info!("Writing to {}", conf_path.display());
            let mut file = File::create(conf_path)?;
            writeln!(file, "{}", serde_yml::to_string(self)?)?;
            Ok(())
        } else {
            Err(Error::msg("Failed to Generate Config"))
        }
    }
}
