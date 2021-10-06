use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("localhost:8000").expect("Unable to bind to port");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Unable to read from stream");

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
