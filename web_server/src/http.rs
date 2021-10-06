use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
enum Method {
    GET,
    POST,
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    target: String,
    version: String,
}

impl Request {
    pub fn new(request_str: &str) -> Result<Request, &str> {
        let request_split: Vec<&str> = request_str.split_whitespace().collect();

        if let [method, target, version] = request_split[..] {
            Ok(Request {
                method: Request::parse_method(method)?,
                target: String::from(target),
                version: String::from(version),
            })
        } else {
            Err("Request does not contain 3 components")
        }
    }

    fn parse_method(method: &str) -> Result<Method, &str> {
        match method {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err("Unknown request method"),
        }
    }
}

#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new(raw_headers: &str) -> Headers {
        let headers = raw_headers
            .split("\r\n")
            .map(Headers::decode_header)
            .map(|header| header.expect("Unable to decode header"));
        Headers {
            0: HashMap::from_iter(headers),
        }
    }

    fn decode_header(raw_header: &str) -> Option<(String, String)> {
        raw_header
            .split_once(": ")
            .map(|(key, value)| (String::from(key), String::from(value)))
    }
}
