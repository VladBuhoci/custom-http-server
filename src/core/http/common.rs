#[derive(Debug)]
pub enum HttpVersion {
    Unknown,
    V0_9(String),
    V1_0(String),
    V1_1(String),
    V2_0(String),
    V3_0(String),
}

impl HttpVersion {
    pub fn parse(http_v_as_str: &str) -> HttpVersion {
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
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct HttpPayload {
    pub value: String,
}
