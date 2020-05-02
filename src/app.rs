use super::scramble::Scramble;
use super::timer::Timer;
use super::event_handler;
use super::event_handler::Event;

use std::sync::mpsc::{Sender, Receiver, RecvError};

use State::*;

pub enum State {
    Idle,
    Inspection(u8),
    Timing,
}

const INSPECTION: bool = true;
pub const INSPECTION_TIME: u8 = 15;

pub struct App {
    pub state: State,
    pub active_timer: Option<Timer>,
    pub scramble: Scramble,
    pub timers: Vec<Timer>,
    pub is_timing: bool,
    pub average_text: Vec<String>,
    pub points: Vec<(f64, f64)>,
    tx: Sender<Event>,
    rx: Receiver<Event>,
}

impl App {
    fn new_scramble(&mut self) {
        self.scramble = Scramble::default();
    }

    pub fn countdown(&mut self) {
        match &mut self.state {
            Inspection(time) => {
                let new_time = *time - 1;
                if new_time == 0 {
                    self.start_timing();
                } else {
                    self.state = Inspection(*time - 1);
                }
            }
            _ => {},
        }
    }

    fn start_timing(&mut self) {
        self.active_timer = Some(Timer::start());
        self.state = Timing;
    }

    fn stop_timing(&mut self) {
        self.active_timer = None;
        self.state = Idle;
    }

    pub fn start_inspection(&mut self) {
        self.state = Inspection(INSPECTION_TIME);

        let tx_inspection = Sender::clone(&self.tx);
        event_handler::spawn_inspection_thread(tx_inspection);
    }

    pub fn toggle(&mut self) {
        match self.state {
            Idle => {
                if INSPECTION {
                    self.start_inspection();
                } else {
                    self.start_timing();
                }
            },
            Inspection(_) => {
                self.start_timing()
            }
            Timing => {
                let mut timer = self.active_timer.unwrap();
                timer.stop();

                self.timers.push(timer);
                self.new_scramble();

                self.stop_timing();
            },
        }
    }

    pub fn process_event(&self) -> Result<Event, RecvError> {
        self.rx.recv()
    }
}

impl Default for App {
    fn default() -> App {
        let (tx, rx) = event_handler::create_handlers();

        App {
            tx,
            rx,
            state: Idle,
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
