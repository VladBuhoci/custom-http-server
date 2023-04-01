use std::io::Read;
use std::net::TcpStream;
use std::str::Lines;

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
        http_req_line = parse_http_request_line(first_buffer_line);
    }

    // Next lines contain headers (one key-value pair per line) until an empty line is met.
    let http_headers= parse_http_header_lines(&mut buffer_lines);

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

/**
 * Expects a string with the format: \<METHOD> \<URI> \<HTTP VERSION>
 */
fn parse_http_request_line(string_to_parse: &str) -> HttpRequestLine {
    let string_tokens = string_to_parse
        .split(" ")
        .collect::<Vec<&str>>();

    HttpRequestLine {
        method: string_tokens[0].parse().unwrap(),
        uri: string_tokens[1].parse().unwrap(),
        version: HttpVersion::parse(string_tokens[2]),
    }
}

/**
 * Expects a string with the format: \<HEADER_NAME> \<HEADER_VALUE>
 */
fn parse_http_header_line(string_to_parse: &str) -> HttpHeader {
    let mut header_pair = string_to_parse.split(":");

    HttpHeader {
        key: header_pair.next().unwrap().trim().to_string(),
        value: header_pair.next().unwrap().trim().to_string(),
    }
}

fn parse_http_header_lines(string_lines_to_parse: &mut Lines<'_>) -> HttpHeaders {
    let mut headers_vec = Vec::with_capacity(5);

    loop {
        let header_line = string_lines_to_parse.next().unwrap();
        if header_line == "" {
            break;
        }

        let header = parse_http_header_line(header_line);
        headers_vec.push(header);
    }

    HttpHeaders {
        headers: headers_vec,
    }
}