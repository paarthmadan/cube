extern crate termion;

mod event_handler;
mod scramble;
mod ui;

use event_handler::{Event, EventHandler};
use scramble::Scramble;

use std::sync::mpsc::channel;


use std::time::{Duration, Instant};

use termion::raw::IntoRawMode;

use std::io;
use tui::backend::TermionBackend;
use tui::Terminal;

pub struct App {
    active_timer: Option<Timer>,
    scramble: Scramble,
    timers: Vec<Timer>,
    is_timing: bool,
    average_text: Vec<String>,
    points: Vec<(f64, f64)>,
}

impl App {
    fn new_scramble(&mut self) {
        self.scramble = Scramble::default();
    }

    fn toggle(&mut self) {
        if self.is_timing {
            let mut timer = self.active_timer.unwrap();
            timer.stop();

            self.timers.push(timer);
            self.new_scramble();

            self.active_timer = None;
            self.is_timing = false;
        } else {
            self.active_timer = Some(Timer::start());
            self.is_timing = true;
        }
    }
}

impl Default for App {
    fn default() -> App {
        App {
            is_timing: false,
            active_timer: None,
            scramble: Scramble::default(),
            timers: Vec::new(),
            average_text: vec![
                "ao5: 25.11".to_string(),
                "ao12: 25.11".to_string(),
                "ao50: 25.11".to_string(),
                "ao100: 25.11".to_string(),
                "stdev: 2.55".to_string(),
            ],
            points: vec![(1.0, 1.0), (2.0, 2.0), (3.0, 1.0), (4.0, 3.0), (5.0, 7.0)],
        }
    }
}

#[derive(Copy, Clone)]
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
