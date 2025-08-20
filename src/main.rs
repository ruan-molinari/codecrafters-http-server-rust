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

    let request = Request::from_buf(str::from_utf8(&buf).unwrap());

    let response = handle_routes(&request);

    let _ = stream.write(&response.as_bytes());
}

fn handle_connection_error(error: impl Error) {
    println!("error: {}", error);
}

// TODO: Create a proper Router for handling routes
// IDEAS:
//  -
fn handle_routes<'r>(ctx: &'r Request) -> Response<'r> {
    let route = ctx
        .target
        .split('/')
        .collect::<Vec<_>>()
        .get(1)
        .unwrap_or(&"")
        .to_string();

    match format!("/{}", route).as_str() {
        "/" => {
            let mut res = Response::new();
            res.status = Status::OK;
            res
        }
        "/echo" => handle_echo(&ctx),
        "/user-agent" => handle_user_agent(&ctx),
        _ => {
            let mut res = Response::new();
            res.status = Status::NotFound;
            res
        }
    }
}

fn handle_echo<'r>(ctx: &Request<'r>) -> Response<'r> {
    let route = ctx.target.split('/').collect::<Vec<_>>();
    let echo = route.get(2).map(|&s| s).unwrap_or("");

    Response {
        status: Status::OK,
        headers: HeaderMap::new(),
        body: Some(echo),
    }
}

fn handle_user_agent<'r>(ctx: &'r Request) -> Response<'r> {
    if let Some(user_agent) = ctx.headers.get("User-Agent") {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "plain/text");
        Response {
            status: Status::OK,
            headers,
            body: Some(&user_agent),
        }
    } else {
        Response {
            status: Status::BadRequest,
            headers: HeaderMap::new(),
            body: None,
        }
    }
}
