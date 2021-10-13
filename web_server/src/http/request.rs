use crate::http::cookies;
use crate::http::headers;
use crate::http::status;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: headers::Headers,
    pub cookies: cookies::Cookies,
    pub body: String,
}

impl Request {
    pub fn new(request: &str) -> Result<Self, (status::Status, &str)> {
        // Split request
        let (request_str, remainder) = request.split_once("\r\n").ok_or((
            status::Status::BadRequest,
            r"Request did not contain a \r\n",
        ))?;
        let (headers, body) = remainder.split_once("\r\n\r\n").ok_or((
            status::Status::BadRequest,
            "Unable to split headers and body form request",
        ))?;

        // Parse headers
        let headers = headers::Headers::new(headers);
        let cookies_raw = match headers.get("Cookie") {
            Some(cookies_raw) => cookies_raw,
            None => "",
        };
        let cookies = cookies::Cookies::new(cookies_raw);

        // Parse request line
        let request_split: Vec<&str> = request_str.split_whitespace().collect();
        if let [method, path, version] = request_split[..] {
            Ok(Self {
                method: Self::parse_method(method)
                    .map_err(|e| (status::Status::NotImplemented, e))?,
                path: String::from(path),
                version: String::from(version),
                headers,
                cookies,
                body: String::from(body),
            })
        } else {
            Err((
                status::Status::BadRequest,
                "Request does not contain 3 components",
            ))
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
