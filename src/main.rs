extern crate termion;
mod event_handler;

use event_handler::{Event, EventHandler};
use std::thread;

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

    fn time(&self) -> Result<Duration, &'static str> {
        match self.end {
            Some(e) => Ok(e.duration_since(self.start)),
            None => Err("No end time"),
        }
    }

    fn elapsed_time(&self) -> Duration {
        self.start.elapsed()
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

    let rx = EventHandler::new();

    let mut active_timer: Option<Timer> = None;

    loop {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        );

        if let Ok(msg) = rx.try_recv() {
            match msg {
                Event::Input(c) => match c {
                    'q' => break,
                    ' ' => {
                        active_timer = match active_timer {
                            Some(mut t) => {
                                let end = Instant::now();
                                t.end = Some(end);
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

        match &active_timer {
            Some(t) => {
                println!("Time: {}", t.elapsed_time().as_millis());
            }
            None => println!("No active timer, press space to start"),
        }

        write!(stdout, "{}", termion::cursor::Goto(1, 2));

        for timer in &app.timers {
            println!("{}\r", timer.time().unwrap().as_millis());
        }

        thread::sleep(Duration::from_millis(10));
    }

    write!(stdout, "{}", termion::cursor::Show);
}
