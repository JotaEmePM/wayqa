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
    pub current_request: Request
}

impl Wayqa {
    pub fn new() -> Wayqa {
        Wayqa {
            input_mode: InputMode::Normal,
            in_project: false,
            project_name: String::from(""),
            saved_info : false,
            current_request: Request::new()
        }
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }
}
