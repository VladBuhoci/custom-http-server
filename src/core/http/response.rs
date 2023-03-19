use std::fmt::{Display, Formatter};
use crate::core::http::common::*;

#[derive(Debug)]
pub struct HttpStatus {
    pub code: i32,
    pub reason: String,
}

#[derive(Debug)]
pub struct HttpStatusLine {
    pub version: HttpVersion,
    pub status: HttpStatus,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_line: HttpStatusLine,
    pub headers: Vec<HttpHeader>,
    pub payload: HttpPayload,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub fn create_response() -> HttpResponse {
    todo!("TODO: see what params to add and how to create this nicely (perhaps with a builder?)");

    // HttpResponse {
    //     status_line: HttpStatusLine {},
    //     headers: vec![],
    //     payload: HttpPayload {},
    // }
}
