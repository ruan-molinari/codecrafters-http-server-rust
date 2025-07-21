mod http;

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{HttpRequest, HttpResponse, HttpStatus};

const HTTP_VERSION: &str = "HTTP/1.1";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection_success(s),
            Err(e) => handle_connection_error(e),
        }
    }
}

fn handle_connection_success(mut stream: std::net::TcpStream) {
    let mut buf = [0; 1024];

    let _ = stream.read(&mut buf).expect("failed to reat to string");

    let request = HttpRequest::new(buf.to_vec());

    let status = handle_routes(&request.target);

    let response = HttpResponse::new(status, vec![], None);

    println!("accepted new connection");
    let _ = stream.write(&response.as_bytes());
}

fn handle_connection_error(error: impl Error) {
    println!("error: {}", error);
}

fn handle_routes(target: &String) -> HttpStatus {
    match target.as_str() {
        "/" => HttpStatus::OK,
        _ => HttpStatus::NotFound,
    }
}
