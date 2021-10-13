use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

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
            let peer_addr = stream.peer_addr().expect("Unable to get peer address");
            println!("Got connection from {}", peer_addr);
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        // Read raw request
        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf).expect("Unable to read from stream");
        let raw_request = String::from_utf8_lossy(&buf[..bytes_read]);

        // Parse request
        let request = request::Request::new(&raw_request);

        // Debug request
        println!("{:#?}", request);

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
