use crate::http::cookies;
use crate::http::headers;

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
    headers: headers::Headers,
    cookies: Option<cookies::Cookies>,
    body: String,
}

impl Request {
    pub fn new(request: &str) -> Result<Request, &str> {
        // Split request
        let (request_str, remainder) = request
            .split_once("\r\n")
            .expect(r"Request did not contain a \r\n");
        let (headers, body) = remainder
            .split_once("\r\n\r\n")
            .expect("Unable to split headers and body form request");

        // Parse headers
        let headers = headers::Headers::new(headers);
        let cookies = match headers.get("Cookie") {
            Some(cookies_raw) => cookies::Cookies::new(cookies_raw),
            None => None,
        };

        // Parse request line
        let request_split: Vec<&str> = request_str.split_whitespace().collect();
        if let [method, target, version] = request_split[..] {
            Ok(Request {
                method: Request::parse_method(method)?,
                target: String::from(target),
                version: String::from(version),
                headers,
                cookies,
                body: String::from(body),
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
