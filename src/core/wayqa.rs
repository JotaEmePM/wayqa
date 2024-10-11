use std::time::Instant;

use ratatui::{style::{Color, Modifier, Style, Stylize}, text::{Line, Span}};

use super::models::request::Request;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Project,
    Request,
    RequestUrl,
    RequestParamsTab,
    RequestResponseTab,
}

pub enum RequestTab {
    Params,
    Authorization,
    Headers,
    Body,
    Settings,
    Response
}

pub struct Wayqa {    
    pub input_mode: InputMode,
    pub in_project: bool,
    pub project_name: String,
    pub saved_info: bool,
    pub current_request: Request,
    pub project_layout_visible: bool,
    pub valid_request: bool,
    pub cursor_visible: bool,

    pub current_request_active_tab: RequestTab,

    // Request inputs
    pub url_cursor_position: usize,

    pub last_toggle_project_layout_visible: Option<Instant>,
    pub last_selected_method: Option<Instant>,
}

impl Wayqa {
    pub fn new() -> Wayqa {
        Wayqa {
            input_mode: InputMode::Normal,
            in_project: false,
            project_name: String::from(""),
            saved_info: false,
            current_request: Request::new(),
            project_layout_visible: false,
            valid_request: false,
            cursor_visible: true,

            current_request_active_tab: RequestTab::Params,

            // Request Input
            url_cursor_position: 0,
            
            last_toggle_project_layout_visible: None,
            last_selected_method: None
        }
    }

    pub fn check_valid_request(&mut self) -> bool {
        false
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }

    pub fn toggle_project_layout(&mut self) {
        self.project_layout_visible = !self.project_layout_visible;
    }

    pub fn move_cursor_url_left(&mut self) {
        let cursor_moved_left = self.url_cursor_position.saturating_sub(1);
        self.url_cursor_position = self.clamp_cursor_request_url(cursor_moved_left);
    }

    pub fn move_cursor_url_right(&mut self) {
        let cursor_moved_right = self.url_cursor_position.saturating_add(1);
        self.url_cursor_position = self.clamp_cursor_request_url(cursor_moved_right);
    }

    pub fn enter_char_request_url(&mut self, new_char: char) {
        let index = self.byte_index_request_url();
        self.current_request.url.insert(index, new_char);
        self.move_cursor_url_right();
    }

    pub fn byte_index_request_url(&self) -> usize {
        self.current_request.url
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.url_cursor_position)
            .unwrap_or(self.current_request.url.len())
    }

    pub fn delete_char_request_url(&mut self) {
        let is_not_cursor_leftmost = self.url_cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.url_cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.current_request.url.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.current_request.url.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.current_request.url = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_url_left();
        }
    }

    fn clamp_cursor_request_url(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.current_request.url.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.url_cursor_position = 0;
    }

    pub fn get_tab_titles(&self) -> Vec<Line> {
        let titles = vec![
            Line::from(vec![
                Span::styled("[1] ", Style::default()).fg(Color::Green).add_modifier(Modifier::BOLD),
                Span::styled("Params", Style::default()).add_modifier(Modifier::BOLD)
            ]),
            Line::from(vec![
                Span::styled("[2] ", Style::default()).fg(Color::Green).add_modifier(Modifier::BOLD),
                Span::styled("Authorization", Style::default()).add_modifier(Modifier::BOLD)
            ]),
            Line::from(vec![
                Span::styled("[3] ", Style::default()).fg(Color::Green).add_modifier(Modifier::BOLD),
                Span::styled("Headers", Style::default()).add_modifier(Modifier::BOLD)
            ]),
            Line::from(vec![
                Span::styled("[4] ", Style::default()).fg(Color::Green).add_modifier(Modifier::BOLD),
                Span::styled("Body", Style::default()).add_modifier(Modifier::BOLD)
            ]),
            Line::from(vec![
                Span::styled("[5] ", Style::default()).fg(Color::Green).add_modifier(Modifier::BOLD),
                Span::styled("Settings", Style::default()).add_modifier(Modifier::BOLD)
            ]),
            Line::from(vec![
                Span::styled("[6] ", Style::default()).fg(Color::Green).add_modifier(Modifier::BOLD),
                Span::styled("Response", Style::default()).add_modifier(Modifier::BOLD)
            ]),
        ];
        titles
    }
}
