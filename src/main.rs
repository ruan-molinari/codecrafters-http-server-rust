use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;

const HTTP_VERSION: &str = "HTTP/1.1";

struct HttpRequest {
    method: String,
    target: String,
    version: String,
    headers: Vec<Header>,
    body: Option<String>,
}

#[derive(Debug, Clone)]
struct Header {
    key: String,
    value: String,
}

impl HttpRequest {
    fn new(buf: Vec<u8>) -> Self {
        let request = String::from_utf8(buf).unwrap();
        let mut lines = request.lines().collect::<Vec<_>>().into_iter();
        let mut status_line = lines.next().unwrap().split_whitespace();

        let mut headers = Vec::new();
        while let Some(header_line) = lines.next() {
            if let Some((key, value)) = header_line.split_once(':') {
                headers.push(Header {
                    key: key.trim().to_string(),
                    value: value.trim().to_string(),
                });
            }
        }
        let body = String::from(lines.collect::<Vec<&str>>().concat());

        HttpRequest {
            method: status_line.next().unwrap().to_string(),
            target: status_line.next().unwrap().to_string(),
            version: status_line.next().unwrap().to_string(),
            headers: headers.to_vec(),
            body: Some(body),
        }
    }
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
    // let mut buf = [0; 1024];
    // let _ = stream.peek(&mut buf).expect("failed peaking request");
    //
    // let req = parse_http_request(String::from_utf8_lossy(&buf).lines().collect());

    // let mut buf = Vec::<u8>::new();
    let mut buf = [0; 1024];

    let len = stream.read(&mut buf).expect("failed to reat to string");

    let request = HttpRequest::new(buf.to_vec());

    let status = handle_routes(&request.target);

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

// fn parse_http_request(request: Vec<u8>) -> HttpRequest {
//     let req_line = request.to_string();
//     let req_line_values: Vec<_> = request[0].to_string().split_whitespace();
//     HttpRequest {
//         method: req_line[0].to_string(),
//         target: req_line[1].to_string(),
//         version: req_line[2].to_string(),
//     }
// }

// fn parse_headers(request: Vec<&str>) -> Vec<(&str, &str)> {
//     request
//         .iter()
//         .skip(2)
//         .take_while(|line| line.is_empty())
//         .filter_map(|line| {
//             if let Some((key, value)) = line.split_once(':') {
//                 Some((key.trim(), value.trim()))
//             } else {
//                 None
//             }
//         })
//         .collect()
// }
