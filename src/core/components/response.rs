use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::core::wayqa::Wayqa;

pub fn render_response_tab(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]);
    let block: [Rect; 1] = horizontal.areas(area);
    let text = Paragraph::new("Hello from response");
    f.render_widget(text, block[0]);
}
