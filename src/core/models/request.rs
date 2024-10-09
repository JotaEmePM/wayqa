pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub url: String,
}



impl Request {

    pub fn new() -> Request {
        Request {
            method: String::from("GET"),
            url: String::from("")
        }
    }

    pub fn new_width_url(url: String) -> Request {
        Request {
            method: String::from("GET"),
            url
        }
    }

    pub fn new_width_method(method: String, url: String) -> Request {
        Request {
            method,
            url
        }
    }
}
