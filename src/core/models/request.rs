use chrono::{DateTime, Local};

#[derive(Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

#[derive(Clone)]
pub enum ResponseCode {
    NONE = 0,
    OK = 200,
    CREATED = 201,
    ACCEPTED = 202,
    NOCONTENT = 204,
    MOVEDPERMANENTLY = 301,
    FOUND = 302,
    NOTMODIFIED = 304,
    BADREQUEST = 400,
    UNAUTHORIZED = 401,
    FORBIDDEN = 403,
    NOTFOUND = 404,
    METHODNOTALLOWED = 405,
    REQUESTTIMEOUT = 408,
    INTERNALSERVERERROR = 500,
    NOTIMPLEMENTED = 501,
    BADGATEWAY = 502,
    SERVICEUNAVAILABLE = 503,
    GATEWAYTIMEOUT = 504,
}

#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub url: String,

    pub code_status: ResponseCode,
    pub time: u128,
    pub size: u64,
    pub last_executed: DateTime<Local>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: Method::GET,
            url: String::from(""),
            code_status: ResponseCode::NONE,
            time: 0,
            size: 0,
            last_executed: Local::now(),
        }
    }

    // pub fn new_width_url(url: String) -> Request {
    //     Request {
    //         method: Method::GET,
    //         url
    //     }
    // }

    // pub fn new_width_method(method: Method, url: String) -> Request {
    //     Request {
    //         method,
    //         url
    //     }
    // }

    // pub fn change_method(&mut self, new_method: Method) {
    //     self.method = new_method;
    // }

    pub fn change_next_method(&mut self) {
        let new_method = match &self.method {
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

    pub async fn execute_request(&self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        let request_builder = match self.method {
            Method::GET => client.get(&self.url),
            Method::POST => client.post(&self.url),
            Method::PUT => client.put(&self.url),
            Method::PATCH => client.patch(&self.url),
            Method::DELETE => client.delete(&self.url),
            Method::HEAD => client.head(&self.url),
            Method::OPTIONS => client.request(reqwest::Method::OPTIONS, &self.url),
        };

        // Add headers, body, params, and authentication as needed
        let request_builder = request_builder
            .header("Content-Type", "application/json")
            .bearer_auth("your_token_here");

        let res = request_builder.send().await?;

        println!("Status: {}", res.status().as_u16());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);

        Ok(())
    }
}
