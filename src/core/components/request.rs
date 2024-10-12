use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph, Tabs},
    Frame,
};

use crate::core::wayqa::{InputMode, RequestTab, Wayqa};

use super::response::render_response_tab;

pub fn render_request_layout(f: &mut Frame, block: Rect, state: &mut Wayqa) {
    let vertical_request = Layout::vertical(
        [
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Fill(1),
        ]
        .as_ref(),
    )
    .margin(2)
    .vertical_margin(1)
    .split(block);

    let method_url_layout =
        Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(vertical_request[0]);

    let method_input = Paragraph::new(state.current_request.get_method_str().to_string())
        // .style(match state.input_mode {
        //     InputMode::RequestMethod => Style::default().fg(Color::Yellow.into()),
        //     _ => Style::default(),
        // })
        .block(Block::bordered().title("Method"));
    f.render_widget(method_input, method_url_layout[0]);

    let url_input = Paragraph::new(state.current_request.url.to_string())
        .style(match state.input_mode {
            InputMode::RequestUrl => Style::default().fg(Color::Yellow.into()),
            _ => Style::default(),
        })
        .block(Block::bordered().title("URL"));
    f.render_widget(url_input, method_url_layout[1]);

    match state.input_mode {
        InputMode::RequestUrl => {
            f.set_cursor_position(Position::new(
                method_url_layout[1].x + state.url_cursor_position as u16 + 1,
                method_url_layout[1].y + 1,
            ));
        }
        _ => {}
    }

    render_request_tab(f, vertical_request[1], vertical_request[2], state);
}

fn render_request_tab(f: &mut Frame, tab_block: Rect, content_block: Rect, state: &mut Wayqa) {
    //let tab_titles = get_tab_titles(&state);

    //state.current_request_active_tab_index ;
    let current_tab = match state.current_request_active_tab {
        RequestTab::Params => 0,
        RequestTab::Authorization => 1,
        RequestTab::Headers => 2,
        RequestTab::Body => 3,
        RequestTab::Settings => 4,
        RequestTab::Response => 5,
    };

    let tab = Tabs::new(get_tab_titles(state))
        .block(Block::default())
        .highlight_style(
            Style::default()
                .fg(ratatui::style::Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .divider(Span::raw(" "))
        .padding("", "")
        .select(current_tab);

    f.render_widget(tab, tab_block);

    match state.current_request_active_tab {
        RequestTab::Params => {
            render_params_tab(f, content_block, state);
        }
        RequestTab::Authorization => render_authorization_tab(f, content_block, state),
        RequestTab::Headers => render_headers_tab(f, content_block, state),
        RequestTab::Body => render_body_tab(f, content_block, state),
        RequestTab::Settings => render_settings_tab(f, content_block, state),
        RequestTab::Response => {
            render_response_tab(f, content_block, state);
        }
        _ => {}
    }
}

pub fn render_params_tab(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]);
    let block: [Rect; 1] = horizontal.areas(area);
    let text = Paragraph::new("Hello from params");
    f.render_widget(text, block[0]);
}

pub fn render_authorization_tab(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]);
    let block: [Rect; 1] = horizontal.areas(area);
    let text = Paragraph::new("Hello from authorization");
    f.render_widget(text, block[0]);
}

pub fn render_headers_tab(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]);
    let block: [Rect; 1] = horizontal.areas(area);
    let text = Paragraph::new("Hello from headers");
    f.render_widget(text, block[0]);
}

pub fn render_body_tab(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]);
    let block: [Rect; 1] = horizontal.areas(area);
    let text = Paragraph::new("Hello from body");
    f.render_widget(text, block[0]);
}

pub fn render_settings_tab(f: &mut Frame, area: Rect, state: &mut Wayqa) {
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]);
    let block: [Rect; 1] = horizontal.areas(area);
    let text = Paragraph::new("Hello from settings");
    f.render_widget(text, block[0]);
}

pub fn get_tab_titles(state: &mut Wayqa) -> Vec<Line> {
    let titles = vec![
        Line::from(vec![
            Span::styled("[1] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Params", Style::default()).add_modifier(Modifier::BOLD),
        ]),
        Line::from(vec![
            Span::styled("[2] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Authorization", Style::default()).add_modifier(Modifier::BOLD),
        ]),
        Line::from(vec![
            Span::styled("[3] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Headers", Style::default()).add_modifier(Modifier::BOLD),
        ]),
        Line::from(vec![
            Span::styled("[4] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Body", Style::default()).add_modifier(Modifier::BOLD),
        ]),
        Line::from(vec![
            Span::styled("[5] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Settings", Style::default()).add_modifier(Modifier::BOLD),
        ]),
        Line::from(vec![
            Span::styled("[6] ", Style::default())
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Span::styled("Response", Style::default()).add_modifier(Modifier::BOLD),
            match state.request_running {
                true => Span::styled(" (Running)", Style::default())
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD),
                _ => { Span::from("")}
            },
        ]),
    ];
    titles
}