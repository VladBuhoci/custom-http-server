use std::fmt::{Display, Formatter};
use std::time::SystemTime;

use crate::core::http::common::*;

#[derive(Debug)]
pub struct HttpStatus {
    pub code: i32,
    pub reason: String,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code, self.reason)
    }
}

#[derive(Debug)]
pub struct HttpStatusLine {
    pub version: HttpVersion,
    pub status: HttpStatus,
}

impl Display for HttpStatusLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.version, self.status)
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status_line: HttpStatusLine,
    pub headers: HttpHeaders,
    pub payload: HttpPayload,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n\n{}", self.status_line, self.headers, self.payload)
    }
}

pub fn create_response(http_version: HttpVersion) -> HttpResponse {
    // todo!("TODO: see what params to add and how to create this nicely (perhaps with a builder?)");

    // TODO: get the payload in a proper way, along with the value for the "Content-Type" header.
    let (payload_str, payload_type) = get_payload_content_and_type();

    let http_status = HttpStatus {
        code: 200,
        reason: String::from("OK"),
    };

    let http_status_line = HttpStatusLine {
        version: http_version,
        status: http_status,
    };

    let http_headers = HttpHeaders::new(5)
        /* NOTE: In theory, the date ought to represent the
         *          moment just before the payload is generated. In practice, the date
         *          can be generated at any time during message origination.
         */
        .add_string("Date", httpdate::fmt_http_date(SystemTime::now()))

        /* NOTE: the origin server's name and version. */
        .add_str("Server", "CustomHttpServer/0.1")

        // Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT ???

        .add_int("Content-Length", payload_str.len() as i32)
        .add_string("Content-Type", payload_type)

        // Accept-Ranges: bytes ???
        // Vary: Accept-Encoding ???

        // headers_vec.push(HttpHeader { key: String::from("Transfer-Encoding"), value: String::from("chunked") });

        /* NOTE: A server that does not support persistent connections MUST send the
         *          "close" connection option in every response message that does not
         *          have a 1xx (Informational) status code.
         */
        // TODO: put an if expr on the HTTP status code to check if it's of type "1xx".
        .add_str("Connection", "close");

    let http_payload = HttpPayload {
        value: String::from(payload_str),
    };

    HttpResponse {
        status_line: http_status_line,
        headers: http_headers,
        payload: http_payload,
    }
}

fn get_payload_content_and_type() -> (String, String) {
    ("<html><body><p>Hello, client!</p></body></html>".into(), "text/html".into())
}
