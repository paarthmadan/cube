extern crate termion;
mod event_handler;
mod scramble;

use event_handler::{Event, EventHandler};
use scramble::Scramble;

use std::thread;

use std::sync::mpsc::{channel};

use std::time::{Duration, Instant};

use termion::raw::IntoRawMode;

use std::io::{stdout, Write};

struct App {
    timers: Vec<Timer>,
    key_pressed: Option<char>,
    is_timing: bool,
}

impl App {
    fn toggle(&mut self) {
        self.is_timing = !self.is_timing;
    }
}

impl Default for App {
    fn default() -> App {
        App {
            timers: Vec::new(),
            key_pressed: None,
            is_timing: false,
        }
    }
}

struct Timer {
    start: Instant,
    end: Option<Instant>,
}

impl Timer {
    fn start() -> Self {
        Timer {
            start: Instant::now(),
            end: None,
        }
    }

    fn stop(&mut self) {
        let t = Instant::now();
        if self.end.is_none() {
            self.end = Some(t);
        }
    }

    fn time(&self) -> Duration {
        match self.end {
            Some(e) => e.duration_since(self.start),
            None => self.start.elapsed(),
        }
    }
}

// Main thread instantiates thread to look after input
// Main thread listens on channel for events
// Main thread handles state change

fn main() {
    let mut app = App::default();

    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide);
    stdout.flush().unwrap();

    let (tx, rx) = channel();
    EventHandler::new(&tx);

    let mut active_timer: Option<Timer> = None;

    loop {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        );

        match &active_timer {
            Some(t) => {
                println!("Time: {}", t.time().as_millis());
            }
            None => {
                let scramble = Scramble::default();
                println!("{}", scramble);
            }
        }

        write!(stdout, "{}", termion::cursor::Goto(1, 2));

        println!("Count: {}\r", app.timers.len());

        for timer in &app.timers {
            println!("{}\r", timer.time().as_millis());
        }

        if let Ok(msg) = rx.recv() {
            match msg {
                Event::Input(c) => match c {
                    'q' => break,
                    ' ' => {
                        active_timer = match active_timer {
                            Some(mut t) => {
                                t.stop();
                                app.timers.push(t);
                                None
                            }
                            None => Some(Timer::start()),
                        };
                    }
                    c => app.key_pressed = Some(c),
                },
            };
        }


        thread::sleep(Duration::from_millis(10));
    }

    write!(stdout, "{}", termion::cursor::Show);
}
