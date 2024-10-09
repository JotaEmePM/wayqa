pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS
}

pub struct Request {
    pub method: Method,
    pub url: String,
}



impl Request {

    pub fn new() -> Request {
        Request {
            method: Method::GET,
            url: String::from("")
        }
    }

    pub fn new_width_url(url: String) -> Request {
        Request {
            method: Method::GET,
            url
        }
    }

    pub fn new_width_method(method: Method, url: String) -> Request {
        Request {
            method,
            url
        }
    }

    pub fn change_method(&mut self, new_method: Method) {
        self.method = new_method;
    }

    pub fn change_next_method (&mut self) {
        let new_method  = match  &self.method {
            Method::GET => Method::POST,
            Method::POST => Method::PUT,
            Method::PUT => Method::PATCH,
            Method::PATCH => Method::DELETE,
            Method::DELETE => Method::HEAD,            
            Method::HEAD => Method::OPTIONS,            
            Method::OPTIONS => Method::GET,            
        };
        self.method = new_method;
    }

    pub fn get_method_str(&self) -> String {
        match &self.method {
            Method::GET => String::from("GET"),
            Method::POST => String::from("POST"),
            Method::PUT => String::from("PUT"),
            Method::PATCH => String::from("PATCH"),
            Method::DELETE => String::from("DELETE"),
            Method::HEAD => String::from("HEAD"),
            Method::OPTIONS => String::from("OPTIONS"),
        }
    }


}
