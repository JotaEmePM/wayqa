pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS
}

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
    GATEWAYTIMEOUT = 504
}

pub struct Request {
    pub method: Method,
    pub url: String,
    pub response: String,
    pub response_code: ResponseCode
}





impl Request {

    pub fn new() -> Request {
        Request {
            method: Method::GET,
            url: String::from(""),
            response: String::from(""),
            response_code: ResponseCode::NONE
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

    pub async fn execute_request(&self) -> Result<(), reqwest::Error> {
        let res = reqwest::get(&self.url).await?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        println!("Body:\n{}", res.text().await?);

        Ok(())


        


    }
}
