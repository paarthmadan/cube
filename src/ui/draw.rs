use super::*;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{
    Axis, Block, Borders, Chart, Dataset, Marker, Paragraph, Row, Table, Text, Widget,
};
use tui::{backend::Backend, Frame};

pub fn draw_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(70),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(size);

    draw_title(f, layout[0]);
    draw_solve_info(f, layout[1], builder::SolveInfo::from(app));
    draw_stats(f, layout[2], builder::Stats::from(app));
    draw_instructions(f, layout[3]);
}

fn draw_title<B: Backend>(f: &mut Frame<B>, section: Rect) {
    Paragraph::new([Text::styled("CubeTimer", Style::default().fg(Color::Blue))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, section);
}

fn draw_solve_info<B: Backend>(f: &mut Frame<B>, section: Rect, data: builder::SolveInfo) {
    let info = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(section);

    Paragraph::new([Text::styled(data.scramble, Style::default().fg(Color::Red))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, info[0]);

    Paragraph::new([Text::styled(data.time, Style::default().fg(Color::Red))].iter())
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(true)
        .render(f, info[1]);

    Paragraph::new(
        [Text::styled(
            data.cube_type,
            Style::default().fg(Color::Red),
        )]
        .iter(),
    )
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .wrap(true)
    .render(f, info[2]);
}

fn draw_stats<B: Backend>(f: &mut Frame<B>, section: Rect, data: builder::Stats) {
    let stats = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ]
            .as_ref(),
        )
        .split(section);

    draw_recent_solves(f, stats[0], data.recent_solves);
    draw_averages(f, stats[1], data.stats);
    draw_graph(f, stats[2], data.graph);
}

fn draw_recent_solves<B: Backend>(f: &mut Frame<B>, section: Rect, recent_solves: String) {
    Paragraph::new(
        [Text::styled(
            recent_solves,
            Style::default().fg(Color::White),
        )]
        .iter(),
    )
    .block(
        Block::default()
            .title("Recent Solves")
            .borders(Borders::ALL),
    )
    .alignment(Alignment::Left)
    .render(f, section);
}

fn draw_averages<B: Backend>(f: &mut Frame<B>, section: Rect, stats: builder::StatsTable) {
    let row_style = Style::default().fg(Color::White);
    Table::new(
        stats.header.iter(),
        stats
            .values
            .iter()
            .map(|v| Row::StyledData(v.into_iter(), row_style)),
    )
    .block(Block::default().title("Statistics").borders(Borders::ALL))
    .header_style(Style::default().fg(Color::Magenta))
    .widths(&[Constraint::Length(8), Constraint::Length(10)])
    .column_spacing(1)
    .render(f, section);
}

fn draw_graph<B: Backend>(f: &mut Frame<B>, section: Rect, graph: builder::GraphInfo) {
    let dataset = Dataset::default()
        .name("All time solves")
        .marker(Marker::Dot)
        .style(Style::default().fg(Color::Cyan))
        .data(&graph.points);

    let x_bounds = graph.x_axis.bounds;
    let x_bounds = [x_bounds.0, x_bounds.1];

    let y_bounds = graph.y_axis.bounds;
    let y_bounds = [y_bounds.0, y_bounds.1];

    Chart::default()
        .block(Block::default().title("Your Solves").borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Solve")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds(x_bounds)
                .labels(&graph.x_axis.labels),
        )
        .y_axis(
            Axis::default()
                .title("Time")
                .title_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::White))
                .bounds(y_bounds)
                .labels(&graph.y_axis.labels),
        )
        .datasets(&[dataset])
        .render(f, section);
}

fn draw_instructions<B: Backend>(f: &mut Frame<B>, section: Rect) {
    let instructions = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(section);

    let top_bottom = Borders::TOP | Borders::BOTTOM;

    Paragraph::new(
        [Text::styled(
            "Press [SPACE] to toggle timer",
            Style::default().fg(Color::Magenta),
        )]
        .iter(),
    )
    .block(Block::default().borders(top_bottom | Borders::LEFT))
    .alignment(Alignment::Center)
    .wrap(true)
    .render(f, instructions[0]);

    Paragraph::new(
        [Text::styled(
            "Press [Q] to exit",
            Style::default().fg(Color::LightMagenta),
        )]
        .iter(),
    )
    .block(Block::default().borders(top_bottom | Borders::RIGHT))
    .alignment(Alignment::Center)
    .wrap(true)
    .render(f, instructions[1]);
}
