use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use std::io::stdin;

pub enum Event {
    Input(char),
    Tick,
}

pub fn create_handlers() -> Receiver<Event> {
    let (tx, rx) = channel();

    let tx_key = Sender::clone(&tx);
    let tx_ticker = Sender::clone(&tx);

    thread::spawn(move || keyboard_handler(tx_key));
    thread::spawn(move || app_ticker(tx_ticker));

    rx
}

fn keyboard_handler(tx: Sender<Event>) {
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
}

fn app_ticker(tx: Sender<Event>) {
    loop {
        if let Err(_) = tx.send(Event::Tick) {
            break;
        };

        thread::sleep(Duration::from_millis(50));
    }
}
