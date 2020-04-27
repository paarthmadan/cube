use super::App;

use tui::{backend::Backend, Frame};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Paragraph, Text, Widget, Block, Borders, SelectableList, Axis, Chart, Dataset, Marker};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Color, Style, Modifier};


pub fn draw<B: Backend>(f: &mut Frame<B>, data: &App) {
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
            .render(f, title);

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

        let time_string = match data.active_timer {
            Some(timer) => timer.time().as_millis().to_string(),
            None => {
                if data.timers.is_empty() {
                    "0.00".to_string()
                } else {
                    data.timers.last().unwrap().time().as_millis().to_string()
                }
            }
        };

        Paragraph::new([Text::styled(&data.scramble.to_string(), Style::default().fg(Color::Red))].iter())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(f, info[0]);


        Paragraph::new([Text::styled(&time_string, Style::default().fg(Color::Red))].iter())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(f, info[1]);

        Paragraph::new([Text::styled(&"3x3".to_string(), Style::default().fg(Color::Red))].iter())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(true)
            .render(f, info[2]);

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


        // TODO: Limit the number of times you pull, this would get wasteful the moment they go off
        // the screen
        let text: Vec<Text> = data.timers.iter().rev().map(|s| Text::styled(s.time().as_millis().to_string() + "\n", Style::default().fg(Color::White))).collect();

        Paragraph::new(text.iter())
            .block(Block::default().title("Recent Solves").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .render(f, recent);

        let text: Vec<Text> = data.average_text.iter().map(|s| Text::styled(s.to_string() + "\n", Style::default().fg(Color::White))).collect();

        Paragraph::new(text.iter())
            .block(Block::default().title("Average").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .render(f, averages);

        let dataset = Dataset::default()
            .name("All time solves")
            .marker(Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&data.points);


        Chart::default()
            .block(Block::default().title("Solves").borders(Borders::ALL))
            .x_axis(Axis::default()
                .title("Solve")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds([0.0, 10.0])
                .labels(&["0.0", "5.0", "10.0"]))
        .y_axis(Axis::default()
            .title("Time")
            .title_style(Style::default().fg(Color::Red))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(&["0.0", "5.0", "10.0"]))
        .datasets(&[dataset])
        .render(f, graph);

}

