mod ui;

use std::io;
use termion::raw::IntoRawMode;
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

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let data = SampleData::new();

    terminal.draw(|mut f| ui::draw(&mut f, data) )
}
