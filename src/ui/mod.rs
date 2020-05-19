mod builder;
pub mod draw;

use super::app::{App, State};
use tui::{backend::Backend, Frame};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    draw::draw_screen(f, app);
}
