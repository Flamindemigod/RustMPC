mod modules;

use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers },
    style::Color,
};
use modules::{
    config::Config,
    crossterm::Crossterm,
    mpd::Mpd,
    ui::{Render, UI},
};

fn main() -> Result<()> {
    stderrlog::new()
        .module(module_path!())
        .verbosity(4)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    let conf = Config::default();
    conf.generate_config()?;
    let mut mpd = Mpd::new(conf.mpd.get_addr()?);
    if let Ok(mut ct) = Crossterm::init() {
        loop {
            mpd.update_loop();
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
                        play if conf.keybinds.play_pause.matches(&play) => mpd.toggle_play(),
                        next if conf.keybinds.next.matches(&next) => mpd.next_song(),
                        prev if conf.keybinds.prev.matches(&prev) => mpd.prev_song(),
                        stop if conf.keybinds.stop.matches(&stop) => mpd.stop_playback(),
                        vol_up if conf.keybinds.vol_up.matches(&vol_up) => mpd.increase_volume(),
                        vol_down if conf.keybinds.vol_down.matches(&vol_down) => {
                            mpd.decrease_volume()
                        }
                        clear_queue if conf.keybinds.clear_queue.matches(&clear_queue) => {
                            mpd.clear_queue()
                        }
                        _ => {}
                    }
                }
            }
            if let Some(song) = mpd.get_current_playing() {
                ct.set_background(
                    modules::ui::Rect {
                        x: 0,
                        y: 0,
                        width: (ct.screen.width as f32 * 0.20).floor() as u32,
                        height: ct.screen.height,
                    },
                    Color::Rgb { r: 127, g: 0, b: 185 },
                );

                ct.set_text(
                    modules::ui::Rect {
                        x: 1,
                        y: 1,
                        width: ct.screen.width,
                        height: 1,
                    },
                    &song.title.unwrap_or_default(),
                    modules::ui::Overflow::Char,
                );
            }
            ct.render_frame()?;
        }
        let _ = ct.destroy();
    }
    Ok(())
}
