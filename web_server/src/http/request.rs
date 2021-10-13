use crate::http::cookies;
use crate::http::headers;

#[derive(Debug)]
enum Method {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Request {
    method: Method,
    target: String,
    version: String,
    headers: headers::Headers,
    cookies: cookies::Cookies,
    body: String,
}

impl Request {
    pub fn new(request: &str) -> Result<Self, &str> {
        // Split request
        println!("Starting request");
        println!("{}", request);
        println!("Ending request");
        let (request_str, remainder) = request
            .split_once("\r\n")
            .ok_or(r"Request did not contain a \r\n")?;
        let (headers, body) = remainder
            .split_once("\r\n\r\n")
            .ok_or("Unable to split headers and body form request")?;

        // Parse headers
        let headers = headers::Headers::new(headers);
        let cookies_raw = match headers.get("Cookie") {
            Some(cookies_raw) => cookies_raw,
            None => "",
        };
        let cookies = cookies::Cookies::new(cookies_raw);

        // Parse request line
        let request_split: Vec<&str> = request_str.split_whitespace().collect();
        if let [method, target, version] = request_split[..] {
            Ok(Self {
                method: Self::parse_method(method)?,
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
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            _ => Err("Unknown request method"),
        }
    }
}
