extern crate termion;

mod event_handler;
mod scramble;
mod timer;
mod ui;
mod app;

use event_handler::{Event, EventHandler};

use std::sync::mpsc::channel;
use termion::raw::IntoRawMode;

use std::io;
use tui::backend::TermionBackend;
use tui::Terminal;

use app::App;

fn main() -> Result<(), io::Error> {
    let mut app = App::default();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    let (tx, rx) = channel();
    EventHandler::new(&tx);

    loop {
        if let Ok(msg) = rx.recv() {
            match msg {
                Event::Input(c) => match c {
                    'q' => break,
                    ' ' => app.toggle(),
                    _ => continue,
                },
                Event::Tick => {
                    terminal.draw(|mut f| ui::draw(&mut f, &app)).unwrap();
                }
            };
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}
