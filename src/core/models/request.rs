pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

// #[derive(Clone)]
pub struct Request {
    method: Method,
    url: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: Method::GET,
            url: String::new(),
        }
    }
    
    // pub fn method(&self) -> &Method {
    //     &self.method
    // }
}
