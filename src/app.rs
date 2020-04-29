use super::timer::Timer;
use super::scramble::Scramble;

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
