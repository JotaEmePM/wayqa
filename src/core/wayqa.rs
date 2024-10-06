pub enum InputMode {
    Normal,
}

pub struct Wayqa<'a> {
    pub input_mode: InputMode,
}

impl<'a> Wayqa<'a> {
    pub fn new() -> Wayqa<'a> {
        Wayqa {
            input_mode: InputMode::Normal,
        }
    }
}
