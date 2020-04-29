use super::event_handler::{Event, EventHandler};
use super::scramble::Scramble;
use super::timer::Timer;

use std::sync::mpsc::{channel, Receiver, RecvError};

pub struct App {
    pub active_timer: Option<Timer>,
    pub scramble: Scramble,
    pub timers: Vec<Timer>,
    pub is_timing: bool,
    pub average_text: Vec<String>,
    pub points: Vec<(f64, f64)>,
    pub rx: Receiver<Event>,
}

impl App {
    fn new_scramble(&mut self) {
        self.scramble = Scramble::default();
    }

    pub fn toggle(&mut self) {
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

    pub fn process_event(&mut self) -> Result<Event, RecvError> {
        self.rx.recv()
    }
}

impl Default for App {
    fn default() -> App {
        let (tx, rx) = channel();
        EventHandler::new(&tx);

        App {
            rx: rx,
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
