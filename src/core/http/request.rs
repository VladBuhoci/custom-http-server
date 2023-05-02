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
    let mut buffer = [0; 1024];

    let buffer_len = tcp_stream.read(&mut buffer).unwrap();

    String::from_utf8(buffer[0..buffer_len].to_vec()).expect("Found invalid UTF-8 while reading the request data")
}

/// # HTTP request example:
///
/// GET /index HTTP/1.1
///
/// Host: localhost:8080
///
/// User-Agent: curl/7.81.0
///
/// Accept: *//*
fn parse_stream_buffer(stream_buffer: String) -> HttpRequest {
    println!("* Processing request: \n**********\n{}\n**********\n", &stream_buffer);

    let mut buffer_lines = stream_buffer.lines();

    // First line in an HTTP request has the following structure: <METHOD> <URI> <HTTP VERSION>
    let http_req_line = parse_http_request_line(&mut buffer_lines);

    // TODO: verify that it is a valid HTTP request.

    // TODO: verify if it is a v0.9 HTTP request, in which case we do not expect any headers or body/payload.. so we stop here and we reject the request.

    // Next lines contain headers (one key-value pair per line) until an empty line is met.
    let http_headers = parse_http_header_lines(&mut buffer_lines);

    // At the end we find the (optional) payload.
    let http_payload = parse_http_payload(&mut buffer_lines);

    HttpRequest {
        request_line: http_req_line,
        headers: http_headers,
        payload: http_payload,
    }
}

///
/// Reads the first encountered line and then returns the result.
///
/// Expects a string with the format: \<METHOD> \<URI> \<HTTP VERSION>
///
fn parse_http_request_line(string_lines_to_parse: &mut Lines<'_>) -> HttpRequestLine {
    let string_to_parse = string_lines_to_parse.next().unwrap();

    let string_tokens = string_to_parse
        .split(" ")
        .collect::<Vec<&str>>();

    HttpRequestLine {
        method: string_tokens[0].parse().unwrap(),
        uri: string_tokens[1].parse().unwrap(),
        version: HttpVersion::parse(string_tokens[2]),
    }
}

///
/// Expects a string with the format: \<HEADER_NAME> \<HEADER_VALUE>
///
fn parse_http_header_line(string_to_parse: &str) -> HttpHeader {
    let mut header_pair = string_to_parse.split(":");

    HttpHeader {
        key: header_pair.next().unwrap().trim().to_string(),
        value: header_pair.next().unwrap().trim().to_string(),
    }
}

///
/// Reads from the first encountered line and stops at the first empty line, then returns the result.
///
/// Expects zero or more lines of strings, each with the format: \<HEADER_NAME> \<HEADER_VALUE>
///
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

///
/// Reads the first encountered line and then returns the result.
///
/// Expects a string with the format: \<PAYLOAD BODY>
///
fn parse_http_payload(string_lines_to_parse: &mut Lines<'_>) -> HttpPayload {
    // In many cases, the payload is not provided (especially in the case of GET requests, so we init the object as an empty body.
    let payload_line = string_lines_to_parse.next().unwrap_or("");

    HttpPayload {
        value: payload_line.to_string(),
    }
}
