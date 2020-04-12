use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, SelectableList};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style, Modifier};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|mut f| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(f.size());

        let header = Block::default().title("AppHeader").borders(Borders::ALL);

        SelectableList::default()
            .block(header)
            .items(&["Solve", "View Statistics"])
            .select(Some(1))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .render(&mut f, layout[0]);


        Block::default()
             .title("Main Area")
             .borders(Borders::ALL)
             .render(&mut f, layout[1]);
    })
}
