// Uncomment this block to pass the first stage
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn build_response_body(body: &str) -> String {
    let status_line = "HTTP/1.1 200 OK\r\n";
    let headers = format!("Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n", body.len());
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

    let path = split_request_line[1];

    let response: String = match path {
        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_owned(),
        _path if path.starts_with("/echo/") => {
            let payload = &path[6..];
            build_response_body(payload)
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
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
