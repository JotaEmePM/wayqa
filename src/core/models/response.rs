use std::collections::HashMap;

use chrono::{DateTime, Local};

use super::request::ResponseCode;

#[derive(Clone)]
pub struct ResponseCookie {

    name: String,
    value: String,
    expires: String,
    max_age: String,
    domain: String,
    http_only: bool,
    secure: bool,
}

#[derive(Clone)]
pub struct ResponseHeader {
    pub name: String,
    pub value: String,
}

#[derive(Clone)]
pub struct Response {
    pub data: String,
    pub data_format: String,

    pub code_status: ResponseCode,
    pub code_text: String,
    pub time: u128,
    pub size: u64,
    pub last_executed: DateTime<Local>,

    pub headers: Vec<ResponseHeader>,
    pub cookies: Vec<ResponseCookie>,

}