use std::io::Read;
use std::net::TcpStream;

#[derive(Debug)]
enum HttpVersion {
    Unknown,
    V0_9(String),
    V1_0(String),
    V1_1(String),
    V2_0(String),
    V3_0(String),
}

impl HttpVersion {
    fn parse(http_v_as_str: &str) -> HttpVersion {
        match http_v_as_str {
            "HTTP/0.9" => HttpVersion::V0_9(String::from(http_v_as_str)),
            "HTTP/1.0" => HttpVersion::V1_0(String::from(http_v_as_str)),
            "HTTP/1.1" => HttpVersion::V1_1(String::from(http_v_as_str)),
            "HTTP/2.0" => HttpVersion::V2_0(String::from(http_v_as_str)),
            "HTTP/3.0" => HttpVersion::V3_0(String::from(http_v_as_str)),
            _ => HttpVersion::Unknown
        }
    }
}

#[derive(Debug)]
struct HttpRequestLine {
    method: String,
    uri: String,
    version: HttpVersion,
}

#[derive(Debug)]
struct HttpHeader {
    key: String,
    value: String,
}

#[derive(Debug)]
struct HttpPayload {
    value: String,
}

#[derive(Debug)]
pub struct HttpRequest {
    request_line: HttpRequestLine,
    headers: Vec<HttpHeader>,
    payload: HttpPayload,
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
    let mut http_headers = Vec::with_capacity(5);
    {
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

            http_headers.push(header);
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