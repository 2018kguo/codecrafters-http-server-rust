#![allow(dead_code)]

// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

enum Endpoint {
    Echo,
    UserAgent,
}

fn build_response_body(body: &str) -> String {
    let status_line = "HTTP/1.1 200 OK\r\n";
    let headers = format!(
        "Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
        body.len()
    );
    let response_body = status_line.to_owned() + &headers + body;
    response_body
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("{:?}", &http_request);

    //     // Request line
    // GET                          // HTTP method
    // /index.html                  // Request target
    // HTTP/1.1                     // HTTP version
    // \r\n                         // CRLF that marks the end of the request line

    // // Headers
    // Host: localhost:4221\r\n     // Header that specifies the server's host and port
    // User-Agent: curl/7.64.1\r\n  // Header that describes the client's user agent
    // Accept: */*\r\n              // Header that specifies which media types the client can accept
    // \r\n                         // CRLF that marks the end of the headers

    // Example response body

    // Status line
    // HTTP/1.1 200 OK
    // \r\n                          // CRLF that marks the end of the status line

    // // Headers
    // Content-Type: text/plain\r\n  // Header that specifies the format of the response body
    // Content-Length: 3\r\n         // Header that specifies the size of the response body, in bytes
    // \r\n                          // CRLF that marks the end of the headers

    // // Response body
    // abc                           // The string from the request
    // // Request body (empty)

    let request_line = &http_request[0];
    let split_request_line: Vec<&str> = request_line.split(" ").collect();

    let mut headers_map: HashMap<&str, &str> = HashMap::new();

    let path = split_request_line[1];
    for line in &http_request[1..] {
        let split_lines: Vec<&str> = line.split(": ").collect();
        if split_lines.len() != 2 {
            panic!("Header malformed");
        }
        headers_map.insert(split_lines[0], split_lines[1]);
    }

    let response: String = match path {
        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_owned(),
        _path if path.starts_with("/echo/") => {
            let payload = &_path[6..];
            build_response_body(payload)
        }
        _path if path.starts_with("/user-agent") => {
            let parsed_user_agent_header = headers_map
                .get("User-Agent")
                .expect("User Agent header not found");
            build_response_body(&parsed_user_agent_header)
        }
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_owned(),
    };
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                std::thread::spawn(|| handle_connection(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
