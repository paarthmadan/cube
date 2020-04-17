use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

use std::io::stdin;

pub enum Event {
    Input(char),
}

pub struct EventHandler {
    tx: Sender<Event>,
}

impl EventHandler {
    pub fn new() -> Receiver<Event> {
        let (tx, rx) = channel();

        EventHandler::create_handler(&tx);

        rx
    }

    fn create_handler(tx: &Sender<Event>) {
        let tx = Sender::clone(tx);
        thread::spawn(move || {
            let stdin = stdin();
            for event in stdin.keys() {
                if let Ok(event) = event {
                    let res = match event {
                        Key::Char(c) => Some(Event::Input(c)),
                        _ => None,
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
}
