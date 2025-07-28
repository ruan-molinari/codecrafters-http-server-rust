mod http;

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{HeaderMap, HttpRequest, HttpResponse, HttpStatus};

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

    let response = handle_routes(&request);

    println!("accepted new connection");
    let _ = stream.write(&response.as_bytes());
}

fn handle_connection_error(error: impl Error) {
    println!("error: {}", error);
}

// TODO: Create a proper Router for handling routes
// IDEAS:
//  -
fn handle_routes(ctx: &HttpRequest) -> HttpResponse {
    let route = ctx
        .target
        .split('/')
        .collect::<Vec<_>>()
        .get(1)
        .unwrap_or(&"")
        .to_string();

    match format!("/{}", route).as_str() {
        "/" => HttpResponse::new(HttpStatus::OK, HeaderMap::new(), None),
        "/echo" => handle_echo(&ctx),
        "/user-agent" => handle_user_agent(&ctx),
        _ => HttpResponse::new(HttpStatus::NotFound, HeaderMap::new(), None),
    }
}

fn handle_echo(ctx: &HttpRequest) -> HttpResponse {
    let route = ctx.target.split('/').collect::<Vec<_>>();
    let echo = route
        .get(2)
        .map(|s| s.to_string())
        .unwrap_or("".to_string());

    HttpResponse::new(HttpStatus::OK, HeaderMap::new(), Some(echo))
}

fn handle_user_agent(ctx: &HttpRequest) -> HttpResponse {
    if let Some(s) = ctx.headers.get("user-agent") {
        HttpResponse::new(HttpStatus::OK, HeaderMap::new(), Some(s.value.clone()))
    } else {
        HttpResponse::new(HttpStatus::BadRequest, HeaderMap::new(), None)
    }
}
