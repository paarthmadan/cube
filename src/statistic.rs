use std::time::Duration;

use Statistic::*;

pub enum Statistic {
    Average(u8),
    Best,
    Worst,
}

impl Statistic {
    pub fn label(&self) -> String {
        match self {
            Average(n) => format!("avg{}", n),
            Best => String::from("best"),
            Worst => String::from("worst"),
        }
    }

    pub fn compute(&self, times: &Vec<Duration>) -> Option<Duration> {
        match self {
            Average(n) => {
                if times.len() < (*n as usize) {
                    None
                } else {
                    let sum = times.iter().map(|x| x.as_secs_f64()).sum::<f64>();
                    let average = sum / (*n as f64);
                    Some(Duration::from_secs_f64(average))
                }
            }
            Worst => times.iter().cloned().max(),
            Best => times.iter().cloned().min(),
        }
    }
}
