use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use crate::http::cookies;
use crate::http::headers;
use crate::http::request;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(addr: &str) -> std::io::Result<Server> {
        Ok(Server {
            listener: TcpListener::bind(addr)?,
        })
    }

    pub fn start(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection Established");
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        // Read raw request
        let mut buf = [0; 1024];
        stream.read(&mut buf).expect("Unable to read from stream");
        let request = String::from_utf8_lossy(&buf);

        // Parse request
        let (request, remainder) = request
            .split_once("\r\n")
            .expect(r"Request did not contain a \r\n");
        let request = request::Request::new(request).expect("Unable to parse request");
        let (headers, body) = remainder
            .split_once("\r\n\r\n")
            .expect("Unable to split headers and body form request");
        let headers = headers::Headers::new(headers);
        let cookies = match headers.get("Cookie") {
            Some(cookies_raw) => cookies::Cookies::new(cookies_raw),
            None => None,
        };

        // Debug request
        println!("{:#?}", request);
        println!("{:#?}", headers);
        println!("{:#?}", cookies);
        println!("{}", body);

        // Send response
        let body = fs::read_to_string("hello.html").expect("Unable to read response file");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream
            .write(response.as_bytes())
            .expect("Unable to write to stream");
        stream.flush().expect("Unable to flush stream");
    }
}
