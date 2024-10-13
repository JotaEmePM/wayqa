use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::core::wayqa::Wayqa;

#[derive(Clone)]
pub enum ResponseTab {
    Body,
    Cookies,
    Headers,
}

pub fn render_response_tab(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let current_tab = match state.current_response_active_tab {
        ResponseTab::Body => 0,
        ResponseTab::Cookies => 1,
        ResponseTab::Headers => 2,
    };

    let respose_layout =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(1)].as_ref()).split(area);

    let tabs = Tabs::new(get_tab_titles(state))
        .block(Block::default())
        .highlight_style(
            Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" "))
        .padding("", "")
        .select(current_tab);

    f.render_widget(tabs, respose_layout[0]);

    match state.current_response_active_tab {
        ResponseTab::Body => render_response_body(f, respose_layout[1], state),
        ResponseTab::Cookies => render_response_cookies(f, respose_layout[1], state),
        ResponseTab::Headers => render_response_headers(f, respose_layout[1], state),
    }
    // let block: [Rect; 1] = horizontal.areas(area);
    // let text = Paragraph::new("Hello from response");
    // f.render_widget(text, block[0]);
}

fn render_response_body(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let block = Block::default()
        .title("Response Body")
        .borders(Borders::ALL);
    f.render_widget(block, area);

    match &state.current_request.response {
        Some(response) => {
            let text = Paragraph::new(response.data.clone());
            f.render_widget(text, area);
        }
        None => {
            let text = Paragraph::new("No response yet");
            f.render_widget(text, area);
        }
    }
}

fn render_response_cookies(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let block = Block::default()
        .title("Response Cookies")
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

fn render_response_headers(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let block = Block::default()
        .title("Response Headers")
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

pub fn get_tab_titles(state: &mut Wayqa) -> Vec<Line> {
    let titles = vec![
        Line::from(vec![
            Span::styled("[7] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Body", Style::default()).add_modifier(Modifier::BOLD),
        ]),
        Line::from(vec![
            Span::styled("[8] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Cookies", Style::default()).add_modifier(Modifier::BOLD),
        ]),
        Line::from(vec![
            Span::styled("[9] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Headers", Style::default()).add_modifier(Modifier::BOLD),
        ]),
    ];
    titles
}
