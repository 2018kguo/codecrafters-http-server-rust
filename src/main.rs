// Uncomment this block to pass the first stage
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

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

    // // Request body (empty)

    let request_line = &http_request[0];
    let split_request_line: Vec<&str> = request_line.split(" ").collect();

    let path = &split_request_line[1];

    let has_path = if path == &"/" { false } else { true };

    let response_404 = "HTTP/1.1 404 Not Found\r\n\r\n";
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    let response_to_use = if has_path { response_404 } else { response };
    stream.write_all(response_to_use.as_bytes()).unwrap();
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
