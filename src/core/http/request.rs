use std::io::Read;
use std::net::TcpStream;

use crate::core::http::common::*;

#[derive(Debug)]
pub struct HttpRequestLine {
    pub method: String,
    pub uri: String,
    pub version: HttpVersion,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub request_line: HttpRequestLine,
    pub headers: HttpHeaders,
    pub payload: HttpPayload,
}

pub fn get_request_from_stream(tcp_stream: &mut TcpStream) -> HttpRequest {
    let in_buffer = get_buffer_from_stream(tcp_stream);
    let http_request = parse_stream_buffer(in_buffer);

    println!("{http_request:#?}");

    http_request
}

fn get_buffer_from_stream(tcp_stream: &mut TcpStream) -> String {
    let mut in_buffer = String::with_capacity(1024);

    tcp_stream.read_to_string(&mut in_buffer);

    in_buffer
}

/// # HTTP request example:
///
/// GET /index HTTP/1.1
/// Host: localhost:8080
/// User-Agent: curl/7.81.0
/// Accept: *//*
fn parse_stream_buffer(stream_buffer: String) -> HttpRequest {
    println!("* Processing request: \n**********\n{}\n**********\n", &stream_buffer);

    let mut buffer_lines = stream_buffer.lines();

    // First line in an HTTP request has the following structure: <METHOD> <URI> <HTTP VERSION>
    let http_req_line;
    {
        let first_buffer_line = buffer_lines.next().unwrap();
        let first_buffer_line_tokens = first_buffer_line
            .split(" ")
            .collect::<Vec<&str>>();

        http_req_line = HttpRequestLine {
            method: first_buffer_line_tokens[0].parse().unwrap(),
            uri: first_buffer_line_tokens[1].parse().unwrap(),
            version: HttpVersion::parse(first_buffer_line_tokens[2]),
        };
    }

    // Next lines contain headers (one key-value pair per line) until an empty line is met.
    let http_headers;
    {
        let mut headers_vec = Vec::with_capacity(5);

        loop {
            let header_line = buffer_lines.next().unwrap();
            if header_line == "" {
                break;
            }

            let mut header_pair = header_line.split(":");
            let header = HttpHeader {
                key: header_pair.next().unwrap().trim().to_string(),
                value: header_pair.next().unwrap().trim().to_string(),
            };

            headers_vec.push(header);
        }

        http_headers = HttpHeaders {
            headers: headers_vec,
        }
    }

    // At the end we find the payload.
    let http_payload;
    {
        let payload_line = buffer_lines.next().unwrap();

        http_payload = HttpPayload {
            value: payload_line.to_string(),
        }
    }

    HttpRequest {
        request_line: http_req_line,
        headers: http_headers,
        payload: http_payload,
    }
}