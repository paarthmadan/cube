use super::app::INSPECTION_TIME;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

use std::io::stdin;

pub enum Event {
    Input(char),
    DrawInterrupt,
    InspectionInterrupt,
}

pub fn spawn_event_threads() -> (Sender<Event>, Receiver<Event>) {
    let (tx, rx) = channel();

    let tx_key = Sender::clone(&tx);
    let tx_ticker = Sender::clone(&tx);

    thread::spawn(move || keyboard_thread(tx_key));
    thread::spawn(move || redraw_interrupt_thread(tx_ticker));

    (tx, rx)
}

fn keyboard_thread(tx: Sender<Event>) {
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

fn redraw_interrupt_thread(tx: Sender<Event>) {
    loop {
        if let Err(_) = tx.send(Event::DrawInterrupt) {
            break;
        };

        thread::sleep(Duration::from_millis(50));
    }
}

pub fn spawn_inspection_thread(tx: Sender<Event>) {
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        if let Err(_) = tx.send(Event::InspectionInterrupt) {
            return;
        }
    });
}
