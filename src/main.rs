extern crate termion;

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

use std::time::{Duration, Instant};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::{
    io::{Write, stdin, stdout},
    process,
};

struct App {
    timers: Vec<Timer>,
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
            timers: Vec::new(),
            key_pressed: None,
            is_timing: false
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
    let (tx, rx) = channel();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide);
    stdout.flush().unwrap();

    event_handler(tx);

    let mut active_timer: Option<Timer> = None;

    loop {
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All);

        if let Ok(msg) = rx.try_recv() {
            match msg {
                Event::Input(c) => {
                    match c {
                        'q' => break,
                        ' ' => {
                            active_timer = match active_timer {
                                Some(mut t) => {
                                    let end = Instant::now();
                                    t.end = Some(end);
                                    app.timers.push(t);
                                    None
                                },
                                None => Some(Timer::start()),
                            };
                        }
                        c => app.key_pressed = Some(c),
                    }
                }
            };
        }

        match &active_timer {
            Some(t) => {
                println!("Time: {}", t.elapsed_time().as_millis());
            },
            None => println!("No active timer, press space to start")
        }

        write!(stdout, "{}", termion::cursor::Goto(1, 2));

        for timer in &app.timers {
            println!("{}\r", timer.time().unwrap().as_millis());
        }

        thread::sleep(Duration::from_millis(10));
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
