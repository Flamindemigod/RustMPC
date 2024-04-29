use anyhow::Result;
use modules::{
    mpd::Mpd,
    ui::{Render, UI},
};
// use crossterm::{
//     cursor, event::{read, Event, KeyCode, KeyEventKind, KeyEventState, KeyModifiers}, queue, style::{self, Print, Stylize}, terminal, ExecutableCommand, QueueableCommand
// };
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

fn test<T>(v: T)
where
    T: UI,
    T: Render,
{
    v.init()
}

fn main() -> Result<()> {
    stderrlog::new()
        .module(module_path!())
        // .quiet(opt.quiet)
        .verbosity(4)
        // .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();
    let mut mpd = Mpd::new("127.0.0.1:6600");
    let list_of_songs = mpd.get_all_songs();
    println!("List of Songs : {:#?}", mpd.get_all_songs());

    mpd.update_loop();
    list_of_songs
        .iter()
        .for_each(|s| mpd.push_into_queue(s.clone()));
    // mpd.push(list_of_songs[0].to_owned());
    mpd.toggle_play();

    // mpd.toggle_play();
    //     let mut stdout = io::stdout();
    //     terminal::enable_raw_mode()?;
    // let size = terminal::size()?;
    //     let mut program = Program { bounds: Rect{x:0, y:0, width: size.0, height: size.1} };
    //     stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    //
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
    //     stdout.flush()?;
    //     terminal::disable_raw_mode()?;
    Ok(())
}
