use std::io;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Paragraph, Text, Widget, Block, Borders, SelectableList};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Color, Style, Modifier};

struct SampleData {
    scramble_string: String,
    time_string: String,
    cube_type_string: String,
    last_10_solves: Vec<f32>,
    points: Vec<(u32, u32)>,
}

impl SampleData {
    fn new() -> Self {
        SampleData {
            scramble_string: "L R U2 F' B' D2 U' L' R' U F B' U2 B' F R2 L2".to_string(),
            time_string: "12.34".to_string(),
            cube_type_string: "3x3".to_string(),
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
        let size = f.size();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(size);

        // TITLE

        let title = layout[0];

        Paragraph::new([Text::styled("CubeTimer", Style::default().fg(Color::Blue))].iter())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, title);

        // CURRENT SOLVE INFO

        let info = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                ].as_ref()
            ).split(layout[1]);

        Paragraph::new([Text::styled(data.scramble_string, Style::default().fg(Color::Red))].iter())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, info[0]);

        Paragraph::new([Text::styled(data.time_string, Style::default().fg(Color::Red))].iter())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, info[1]);

        Paragraph::new([Text::styled(data.cube_type_string, Style::default().fg(Color::Red))].iter())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(&mut f, info[2]);

        // STATS SECTION

        let stats = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                ].as_ref()
            ).split(layout[2]);


        let recent = stats[0];
        let averages = stats[1];
        let graph = stats[2];


        let text: Vec<Text> = data.last_10_solves.iter().map(|s| Text::styled(s.to_string() + "\n", Style::default().fg(Color::White))).collect();

        Paragraph::new(text.iter())
            .block(Block::default().title("Recent Solves").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .render(&mut f, recent);
    })
}
