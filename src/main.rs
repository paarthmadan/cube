extern crate termion;

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::{
    io::{Write, stdin, stdout},
    process,
};

struct App {
    key_pressed: Option<char>,
    is_timing: bool
}

impl App {
    fn toggle(&mut self) {
        self.is_timing = !self.is_timing;
    }
}

impl Default for App {
    fn default() -> App {
        App {
            key_pressed: None,
            is_timing: false
        }
    }
}

// Main thread instantiates thread to look after input
// Main thread listens on channel for events
// Main thread handles state change

fn main() {
    let mut app = App::default();

    let mut stdout = stdout().into_raw_mode().unwrap();
    let (tx, rx) = channel();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide);
    stdout.flush().unwrap();

    event_handler(tx);

    loop {
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine);
        if let Ok(msg) = rx.recv() {
            match msg {
                Event::Input(c) => {
                    match c {
                        'q' => break,
                        ' ' => app.toggle(),
                        c => app.key_pressed = Some(c),
                    }
                }
            };
        }



        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show);
}

enum Event {
    Input(char),
}

impl Event {
}


fn event_handler(tx: Sender<Event>) {
    thread::spawn(move || {
        let stdin = stdin();
        for event in stdin.keys() {
            if let Ok(event) = event {
                let res = match event {
                    Key::Char(c) => Some(Event::Input(c)),
                    _ => None
                };
                if let Some(ret) = res {
                    if let Err(_) = tx.send(ret) {
                        return;
                    }
                }
            }
        }
    });
}
