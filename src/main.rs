extern crate gus;
extern crate gus_http;

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use gus::request::Request;
use gus::response::Response;
use gus_http::{Header, HeaderMap, Status};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(s) => handle_connection_success(&s),
            Err(e) => handle_connection_error(e),
        });
    }
}

fn handle_connection_success(mut stream: &std::net::TcpStream) {
    println!("accepted new connection");

    let mut buf = [0; 1024];

    let _ = stream.read(&mut buf).expect("failed to read to string");

    let request = Request::new(buf.to_vec());

    let response = handle_routes(&request);

    let _ = stream.write(&response.as_bytes());
}

fn handle_connection_error(error: impl Error) {
    println!("error: {}", error);
}

// TODO: Create a proper Router for handling routes
// IDEAS:
//  -
fn handle_routes(ctx: &Request) -> Response {
    let route = ctx
        .target
        .split('/')
        .collect::<Vec<_>>()
        .get(1)
        .unwrap_or(&"")
        .to_string();

    match format!("/{}", route).as_str() {
        "/" => Response::new(Status::OK, HeaderMap::new(), None),
        "/echo" => handle_echo(&ctx),
        "/user-agent" => handle_user_agent(&ctx),
        _ => Response::new(Status::NotFound, HeaderMap::new(), None),
    }
}

fn handle_echo(ctx: &Request) -> Response {
    let route = ctx.target.split('/').collect::<Vec<_>>();
    let echo = route
        .get(2)
        .map(|s| s.to_string())
        .unwrap_or("".to_string());

    Response::new(Status::OK, HeaderMap::new(), Some(echo))
}

fn handle_user_agent(ctx: &Request) -> Response {
    if let Some(s) = ctx.headers.get("user-agent") {
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-length".to_string(),
            Header::new("Content-Length".to_string(), "plain/text".to_string()),
        );
        Response::new(Status::OK, headers, Some(s.value.clone()))
    } else {
        Response::new(Status::BadRequest, HeaderMap::new(), None)
    }
}
