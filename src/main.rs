pub mod controller;
pub mod database;
pub mod entity;

// TODO: Have dynamic mapping for controllers to be called based on path and HTTP method
// TODO: read payload of POST HTTP requests
// TODO: handle query params

use crate::{
    controller::{get_users, user::fake},
    database::connect,
};

use core::str;
use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Define request handlers and pass those to handle_connection
    let request_handlers: Vec<(&str, &str, fn(&mut TcpStream))> = vec![
        ("GET", "/users", get_users),
        ("GET", "/fake", fake),
        ("GET", "/test", fake),
    ];

    let _ = connect();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🚀 Started server on port 7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // TODO: add multi threading: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
        handle_connection(stream, &request_handlers);
    }
}

fn handle_connection(
    mut stream: TcpStream,
    request_handlers: &Vec<(&str, &str, fn(&mut TcpStream))>,
) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let (http_method, meta_info) = http_request[0].split_at(http_request[0].find(" ").unwrap());
    let (path, _) = meta_info
        .trim()
        .split_at(meta_info.trim().find(" ").unwrap());
    println!("Incoming Request: {} {}", http_method, path);
    let request_handler = request_handlers
        .into_iter()
        .find(|t| t.0 == http_method && t.1 == path);
    if request_handler.is_some() {
        request_handler.unwrap().2(&mut stream);
    } else if http_method == "GET" && fs::metadata(format!("static{path}")).is_ok() {
        respond_with_file(&mut stream, path);
    } else {
        repond_with_error(&mut stream);
    }
}

fn map_to_content_type(file_extension: &str) -> &str {
    return match file_extension {
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "css" => "text/css",
        "html" => "text/html",
        "js" => "application/javascript",
        &_ => "",
    };
}

fn respond_with_file(stream: &mut TcpStream, path: &str) {
    let status_line = "HTTP/1.1 200 OK";
    let file_extension = path.split(".").last().unwrap();
    let content_type = map_to_content_type(file_extension);
    if file_extension == "css" || file_extension == "html" || file_extension == "js" {
        let contents = fs::read_to_string(format!("static{path}")).unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if file_extension == "jpg" || file_extension == "jpeg" || file_extension == "png" {
        let mut file = File::open(format!("static{path}")).expect("Could read file...");
        let length = *&file.metadata().unwrap().len();
        let mut buffer = vec![0; length as usize];
        let _ = file.read(&mut buffer).expect("could read file into buffer");
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type}\r\n\r\n"
        );
        stream.write(response.as_bytes()).unwrap();
        stream.write(&buffer).unwrap();
        stream.flush().unwrap();
    } else {
        panic!("Unsupported file format: {}", file_extension); // TODO: better error handling
    }
}

// TODO: have generic error response method
fn repond_with_error(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("static/404.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
