use std::io::Write;

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{
        Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton,
        MouseEvent, MouseEventKind,
    },
    style::{Color, Print, SetBackgroundColor},
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};
use modules::{
    crossterm::Crossterm,
    mpd::Mpd,
    ui::{Render, UI},
};
mod modules;

// #[derive(Debug)]
// struct Rect {
//     x: u16,
//     y: u16,
//     width: u16,
//     height: u16,
// }
//
// #[derive(Debug)]
// struct Program{
//     bounds: Rect,
// }

fn main() -> Result<()> {
    stderrlog::new()
        .module(module_path!())
        // .quiet(opt.quiet)
        .verbosity(4)
        // .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();
    // let mut mpd = Mpd::new("127.0.0.1:6600");
    // let list_of_songs = mpd.get_all_songs();
    // println!("List of Songs : {:#?}", mpd.get_all_songs());

    // mpd.update_loop();
    // list_of_songs
    //     .iter()
    //     .for_each(|s| mpd.push_into_queue(s.clone()));
    // mpd.toggle_shuffle();
    // mpd.push(list_of_songs[0].to_owned());
    // mpd.toggle_play();
    let mut bg = SetBackgroundColor(Color::Black);
    if let Ok(mut ct) = Crossterm::init() {
        loop {
            if ct.is_event_ready() {
                if let Ok(event) = ct.read_event() {
                    match event {
                        Event::Key(KeyEvent {
                            code: KeyCode::Char('q'),
                            ..
                        })
                        | Event::Key(KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        })
                        | Event::Key(KeyEvent {
                            code: KeyCode::Esc, ..
                        }) => break,
                        Event::Mouse(MouseEvent {
                            kind: MouseEventKind::Down(MouseButton::Left),
                            ..
                        }) => {
                            bg = SetBackgroundColor(Color::DarkRed);
                        },
                        Event::Mouse(MouseEvent {
                            kind: MouseEventKind::Up(MouseButton::Left),
                            ..
                        }) => {
                            bg = SetBackgroundColor(Color::Reset);
                        }

                        _ => {}
                    }
                }
            }
            ct.stdout
                .queue(MoveTo(0, 0))
                .and_then(|q| {
                    q.queue(bg)?.queue(Print(format!(
                        "{x}:{y}@{w}x{h}", 
                        x = ct.screen.x,
                        y = ct.screen.y,
                        w = ct.screen.width,
                        h = ct.screen.height,
                    )))
                })?
                .queue(terminal::Clear(ClearType::UntilNewLine))?;
            ct.stdout.flush()?;
        }
        let _ = ct.destroy();
    }
    //     loop {
    //         //Event Loop
    //         if let event = read()? {
    //             if event.eq(&Event::Key(crossterm::event::KeyEvent {
    //                 code: KeyCode::Char('c'),
    //                 modifiers: KeyModifiers::CONTROL,
    //                 kind: KeyEventKind::Press,
    //                 state: KeyEventState::empty(),
    //             })) {
    //                 break;
    //             }
    //             match event {
    //             Event::FocusGained => println!("FocusGained"),
    //             Event::FocusLost => println!("FocusLost"),
    //             Event::Key(event) => match event.code {
    //                 // KeyCode::Char('q') => break,
    //                 _ => println!("{:?}", event),
    //             },
    //             Event::Mouse(event) => println!("{:?}", event),
    //             Event::Paste(data) => println!("{:?}", data),
    //             Event::Resize(width, height) => {
    //                 program.bounds.width = width;
    //                 program.bounds.height = height;
    //
    //                 queue!(stdout, cursor::MoveTo(0,0), Print(format!("width: {width} X height: {height}")))?;
    //                 stdout.flush()?;
    //             },
    //         }
    //
    //         }
    //         //Draw Loop
    //     }
    //
    Ok(())
}
