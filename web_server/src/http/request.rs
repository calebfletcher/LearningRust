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
