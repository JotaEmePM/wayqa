use std::time::Instant;

use super::models::request::Request;

pub enum InputMode {
    Normal,
    Project,
    Request,
    RequestMethod,
    RequestUrl
}

pub struct Wayqa {
    pub input_mode: InputMode,
    pub in_project: bool,
    pub project_name: String,
    pub saved_info : bool,
    pub current_request: Request,
    pub project_layout_visible: bool,
    pub last_toggle_project_layout_visible: Option<Instant>,
    pub valid_request: bool,    
}

impl Wayqa{
    pub fn new() -> Wayqa {
        Wayqa {
            input_mode: InputMode::Normal,
            in_project: false,
            project_name: String::from(""),
            saved_info : false,
            current_request: Request::new(),
            project_layout_visible: false,
            last_toggle_project_layout_visible: None,
            valid_request: false            
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
}
