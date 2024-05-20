#![allow(dead_code)]

// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

enum ContentType {
    Text,
    File,
}

#[derive(PartialEq, Eq)]
enum EncodingType {
    Gzip,
}

fn build_response_body(
    body: &[u8],
    content_type: ContentType,
    encoding_type: Option<EncodingType>,
) -> Vec<u8> {
    let status_line = "HTTP/1.1 200 OK\r\n";
    let content_type_str = match content_type {
        ContentType::File => "application/octet-stream",
        ContentType::Text => "text/plain",
    };
    let headers = format!(
        "Content-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type_str,
        body.len()
    );
    let mut response_body_bytes: Vec<u8> = Vec::new();
    let content_encoding_header = if encoding_type == Some(EncodingType::Gzip) {
        "Content-Encoding: gzip\r\n"
    } else {
        ""
    };
    response_body_bytes
        .extend((status_line.to_owned() + content_encoding_header + &headers).as_bytes());
    response_body_bytes.extend(body);
    response_body_bytes
}

fn split_stream(mut stream: &TcpStream) -> Result<Vec<String>, &str> {
    let mut buf = [0u8; 1024];
    let _ = stream.read(&mut buf).unwrap();
    let string = String::from_utf8(buf.to_vec()).unwrap();
    let lines = string
        .split("\r\n")
        .map(|s| s.to_string())
        .filter(|x| !x.is_empty())
        .map(|x| x.replace('\0', ""))
        .collect::<Vec<_>>();
    Ok(lines)
}

fn handle_connection(mut stream: TcpStream, args: Vec<String>) {
    let http_request: Vec<String> = split_stream(&stream).unwrap();
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

    let verb = split_request_line[0];
    let path = split_request_line[1];
    let mut body: Option<String> = None;

    for line in &http_request[1..] {
        let split_lines: Vec<&str> = line.split(": ").collect();
        if split_lines.len() == 2 {
            headers_map.insert(split_lines[0], split_lines[1]);
        } else {
            body = Some(line.to_string());
        }
    }

    println!("{:?}", body);
    let ok_response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec();
    let not_found_response = "HTTP/1.1 404 Not Found\r\n\r\n".as_bytes().to_vec();
    let created_response = "HTTP/1.1 201 Created\r\n\r\n".as_bytes().to_vec();

    let encoding_type = headers_map.get("Accept-Encoding");
    let encoding_type_values: Vec<&str> = if let Some(etype) = encoding_type {
        (*etype).split(", ").collect()
    } else {
        Vec::new()
    };

    let encoding_type_enum = if encoding_type_values.contains(&"gzip") {
        Some(EncodingType::Gzip)
    } else {
        None
    };

    let response: Vec<u8> = match (path, verb, encoding_type_enum) {
        ("/", _, _) => ok_response,
        (_path, _, _encoding_type_enum) if path.starts_with("/echo/") => {
            let payload = &_path[6..];
            build_response_body(payload.as_bytes(), ContentType::Text, _encoding_type_enum)
        }
        (_path, _, _encoding_type_enum) if path.starts_with("/user-agent") => {
            let parsed_user_agent_header = headers_map
                .get("User-Agent")
                .expect("User Agent header not found");
            build_response_body(
                &parsed_user_agent_header.as_bytes(),
                ContentType::Text,
                _encoding_type_enum,
            )
        }
        (_path, _, _encoding_type_enum) if path.starts_with("/files/") && verb == "GET" => {
            let file_name = &_path[7..];
            let directory_flag = &args
                .iter()
                .position(|arg| arg == "--directory")
                .expect("no directory flag found");
            let directory_arg = &args[directory_flag + 1];

            // Concatenate the directory path and file name
            let file_path = format!("{}/{}", directory_arg, file_name);

            // Open the file
            let file_result = fs::File::open(file_path);

            match file_result {
                Ok(mut _file_result) => {
                    let mut contents: Vec<u8> = Vec::new();
                    _file_result.read_to_end(&mut contents).unwrap();
                    build_response_body(
                        &contents.as_slice(),
                        ContentType::File,
                        _encoding_type_enum,
                    )
                }
                Err(_) => not_found_response,
            }
        }
        (_path, _, _encoding_type_enum) if path.starts_with("/files/") && verb == "POST" => {
            let file_name = &_path[7..];
            let directory_flag = &args
                .iter()
                .position(|arg| arg == "--directory")
                .expect("no directory flag found");
            let directory_arg = &args[directory_flag + 1];
            let contents = body.expect("no body provided in payload");
            let mut file = File::create(directory_arg.to_owned() + file_name).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
            created_response
        }
        _ => not_found_response,
    };
    println!("DONE!");
    stream.write_all(response.as_slice()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let args: Vec<String> = env::args().collect();

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let cloned_args = args.clone();
                std::thread::spawn(|| handle_connection(_stream, cloned_args));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
