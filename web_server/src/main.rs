use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:8000").expect("Unable to bind to port");
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection Established");
    }
}
