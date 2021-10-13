#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod http;

fn main() {
    let server = http::server::Server::new("localhost:8000").expect("Unable to bind to port");
    server.start();
}
