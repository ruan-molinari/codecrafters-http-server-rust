use std::error::Error;
use std::io::Write;
use std::net::TcpListener;

const HTTP_VERSION: &str = "HTTP/1.1";

struct HttpRequest {
    method: String,
    target: String,
    version: String,
}

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
    let _ = stream.peek(&mut buf).expect("failed peaking request");

    let req = parse_http_request(String::from_utf8_lossy(&buf).lines().collect());

    let status = handle_routes(&req.target);

    let binding = [HTTP_VERSION, " ", status, "\r\n\r\n"].concat();
    let response = binding.as_bytes();

    println!("accepted new connection");
    let _ = stream.write(response);
}

fn handle_connection_error(error: impl Error) {
    println!("error: {}", error);
}

fn handle_routes(target: &String) -> &str {
    match target.as_str() {
        "/" => "200 OK",
        _ => "404 Not Found",
    }
}

fn parse_http_request(request: Vec<&str>) -> HttpRequest {
    let req_line = request[0].split(' ').collect::<Vec<&str>>();
    HttpRequest {
        method: req_line[0].to_string(),
        target: req_line[1].to_string(),
        version: req_line[2].to_string(),
    }
}
