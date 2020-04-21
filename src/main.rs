use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Paragraph, Text, Widget, Block, Borders, SelectableList};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Color, Style, Modifier};

struct SampleData {
    scramble_string: String,
    time_string: String,
    last_10_solves: Vec<f32>,
    points: Vec<(u32, u32)>,
}

impl SampleData {
    fn new() -> Self {
        SampleData {
            scramble_string: "L R U2 F' B' D2 U' L' R' U F B' U2 B' F R2 L2".to_string(),
            time_string: "12.34".to_string(),
            last_10_solves: vec![12.34, 6.54, 5.55, 6.24, 21.54, 10.00, 64.32, 10f32, 4.44, 3.33],
            points: vec![
                (1, 1),
                (2, 2),
                (3, 1),
                (4, 3),
                (5, 7),
            ]
        }
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let data = SampleData::new();

    terminal.draw(|mut f| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .split(f.size());

        let a_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ].as_ref()
            )
            .margin(1)
            .split(layout[0]);


        let header = Block::default().title("Solving Area").borders(Borders::ALL).render(&mut f, layout[0]);
        let stats = Block::default()
             .title("Statistics")
             .borders(Borders::ALL)
             .render(&mut f, layout[1]);

        let b_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Max(1),
                    Constraint::Max(1),
                ].as_ref()
            )
            .margin(1)
            .split(a_chunks[0]);

        let text = [Text::styled(data.scramble_string, Style::default().fg(Color::Blue))];

        let scramble_text = Paragraph::new(text.iter())
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, b_chunks[0]);

        let text = [Text::styled(data.time_string, Style::default().fg(Color::Red))];

        let time_text = Paragraph::new(text.iter())
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, b_chunks[1]);
    })
}
