use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use crate::http::request;
use crate::http::status;
use crate::http::threadpool;

pub struct Server {
    listener: TcpListener,
    pool: threadpool::ThreadPool,
}

impl Server {
    pub fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            pool: threadpool::ThreadPool::new(4)?,
        })
    }

    pub fn start(&self) {
        for stream in self.listener.incoming().take(2) {
            let stream = stream.unwrap();
            let peer_addr = stream.peer_addr().expect("Unable to get peer address");
            println!("Got connection from {}", peer_addr);
            self.pool.execute(|| {
                Self::handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        // Read raw request
        let mut buf = [0; 1024];
        let bytes_read = stream.read(&mut buf).expect("Unable to read from stream");
        let raw_request = String::from_utf8_lossy(&buf[..bytes_read]);

        // Parse request
        let request = request::Request::new(&raw_request);

        let request = match request {
            Ok(request) => request,
            Err((status, msg)) => {
                Self::send_response(stream, status, msg);
                return;
            }
        };

        // Debug request
        // println!("{:#?}", request);

        let (status, body) = match request.path.as_str() {
            "/" => (
                status::Status::Ok,
                fs::read_to_string("hello.html").expect("Unable to read response file"),
            ),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                (status::Status::Ok, String::from("Sleep Endpoint"))
            }
            path => (status::Status::NotFound, format!("Not found: {}", path)),
        };

        Self::send_response(stream, status, &body);
    }

    fn send_response(mut stream: TcpStream, status: status::Status, body: &str) {
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            status.as_response_code(),
            body.len(),
            body
        );
        stream
            .write_all(response.as_bytes())
            .expect("Unable to write to stream");
        stream.flush().expect("Unable to flush stream");
    }
}
