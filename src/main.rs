mod http;

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{Header, HttpRequest, HttpResponse, HttpStatus};

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

    let _ = stream.read(&mut buf).expect("failed to read to string");

    let request = HttpRequest::new(buf.to_vec());

    let response = handle_routes(&request.target);

    println!("accepted new connection");
    let _ = stream.write(&response.as_bytes());
}

fn handle_connection_error(error: impl Error) {
    println!("error: {}", error);
}

fn handle_routes(target: &String) -> HttpResponse {
    let route = target
        .split('/')
        .collect::<Vec<_>>()
        .get(1)
        .unwrap_or(&"")
        .to_string();

    match format!("/{}", route).as_str() {
        "/" => HttpResponse::new(HttpStatus::OK, vec![], None),
        "/echo" => handle_echo(target),
        _ => HttpResponse::new(HttpStatus::NotFound, vec![], None),
    }
}

fn handle_echo(target: &String) -> HttpResponse {
    let route = target.split('/').collect::<Vec<_>>();
    let echo = route
        .get(2)
        .map(|s| s.to_string())
        .unwrap_or("".to_string());

    HttpResponse::new(
        HttpStatus::OK,
        vec![
            Header {
                key: "Content-Type".to_string(),
                value: "text/plain".to_string(),
            },
            Header {
                key: "Content-Length".to_string(),
                value: echo.len().to_string(),
            },
        ],
        Some(echo),
    )
}
