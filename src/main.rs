extern crate termion;

mod event_handler;
mod ui;
mod scramble;

use event_handler::{Event, EventHandler};
use scramble::Scramble;

use std::sync::mpsc::channel;

use std::time::{Duration, Instant};

use termion::raw::IntoRawMode;

use std::io::{stdout, Write};

use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;

pub struct SampleData {
    scramble_string: String,
    time_string: String,
    cube_type_string: String,
    last_10_solves: Vec<f32>,
    average_text: Vec<String>,
    points: Vec<(f64, f64)>,
}



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

impl SampleData {
    fn new() -> Self {
        SampleData {
            scramble_string: "L R U2 F' B' D2 U' L' R' U F B' U2 B' F R2 L2".to_string(),
            time_string: "12.34".to_string(),
            cube_type_string: "3x3".to_string(),
            last_10_solves: vec![12.34, 6.54, 5.55, 6.24, 21.54, 10.00, 64.32, 10f32, 4.44, 3.33],
            average_text: vec![
                "ao5: 25.11".to_string(),
                "ao12: 25.11".to_string(),
                "ao50: 25.11".to_string(),
                "ao100: 25.11".to_string(),
                "stdev: 2.55".to_string(),
            ],
            points: vec![
                (1.0, 1.0),
                (2.0, 2.0),
                (3.0, 1.0),
                (4.0, 3.0),
                (5.0, 7.0),
            ]
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
    let mut tick_count = 0;
    let mut scramble = Scramble::default();

    loop {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        );

        if let Ok(msg) = rx.recv() {
            match msg {
                Event::Input(c) => match c {
                    'q' => break,
                    ' ' => {
                        active_timer = match active_timer {
                            Some(mut t) => {
                                t.stop();
                                app.timers.push(t);
                                scramble = Scramble::default();
                                None
                            }
                            None => Some(Timer::start()),
                        };
                    }
                    c => app.key_pressed = Some(c),
                },
                Event::Tick => {
                    tick_count += 1;
                    match &active_timer {
                        Some(t) => {
                            println!("Time: {}", t.time().as_millis());
                        }
                        None => {
                            println!("{}", scramble);
                        }
                    }

                    write!(stdout, "{}", termion::cursor::Goto(1, 2));

                    println!("Count: {}, Tick Count: {}\r", app.timers.len(), tick_count);

                    for timer in &app.timers {
                        println!("{}\r", timer.time().as_millis());
                    }
                }
            };
        }
    }

    write!(stdout, "{}", termion::cursor::Show);
}
