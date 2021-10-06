use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

mod http;

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

    let request = String::from_utf8_lossy(&buf);

    let (request, remainder) = request
        .split_once("\r\n")
        .expect(r"Request did not contain a \r\n");

    let request = http::Request::new(request).expect("Unable to parse request");
    let (headers, body) = remainder
        .split_once("\r\n\r\n")
        .expect("Unable to split headers and body form request");

    let headers = http::Headers::new(headers);

    println!("{:#?}", request);
    println!("{:#?}", headers);
    println!("{}", body);

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
