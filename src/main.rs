pub mod controller;
pub mod database;
pub mod dto;
pub mod entity;
pub mod util;

// TODO: read payload of POST HTTP requests
// TODO: handle query params

use core::str;
use std::{
    fs::{self, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::dto::Request;

fn main() {
    //
    // Define request handlers and pass those to handle_connection
    let request_handlers: Vec<(&str, &str, fn(&mut Request, &mut TcpStream))> = vec![
        ("GET", "/users", controller::get_users),
        ("POST", "/users", controller::add_user),
        ("GET", "/fake", controller::fake),
    ];

    let _ = database::connect();
    // TODO: have better error handling if database connection fails

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🚀 Started server on port 7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // TODO: add multi threading: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
        handle_connection(stream, &request_handlers);
    }
}

fn empty_string() -> String {
    return "".to_owned();
}

// From: https://stackoverflow.com/questions/71478238/rust-tcpstream-reading-http-request-sometimes-lose-the-body
//
fn parse_request(stream: &TcpStream) -> Request {
    let mut request = Request {
        path: empty_string(),
        http_method: empty_string(),
        header: empty_string(),
        payload: empty_string(),
    };
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut name = String::new();
    loop {
        let r = reader.read_line(&mut name).unwrap();
        if r < 3 {
            //detect empty line
            break;
        }
    }
    let mut size = 0;
    let linesplit = name.split("\n");
    let mut index = 0;
    let mut header = "".to_owned();
    for line in linesplit {
        header.push_str(line);
        if index == 0 {
            let (http_method, meta_info) = line.split_at(line.find(" ").unwrap());
            let (path, _) = meta_info
                .trim()
                .split_at(meta_info.trim().find(" ").unwrap());
            request.http_method = http_method.to_owned();
            request.path = path.to_owned();
        }
        if line.starts_with("Content-Length") {
            let sizeplit = line.split(":");
            for s in sizeplit {
                if !(s.starts_with("Content-Length")) {
                    size = s.trim().parse::<usize>().unwrap(); //Get Content-Length
                }
            }
        }
        index += 1;
    }
    request.header = header;
    let mut buffer = vec![0; size];
    reader.read_exact(&mut buffer).unwrap();
    request.payload = std::str::from_utf8(&buffer).unwrap().to_owned();
    return request;
}

fn handle_connection(
    mut stream: TcpStream,
    request_handlers: &Vec<(&str, &str, fn(&mut Request, &mut TcpStream))>,
) {
    let mut request = parse_request(&stream);

    //  Get the HTTP method and path from HTTP header
    //  Call the correct request controller depending on path and HTTP method
    //  If no controller exists, try to find a file that matches and return that
    //  If no file exists, return a 404 error
    let request_handler = request_handlers
        .into_iter()
        .find(|t| t.0.to_owned() == request.http_method && t.1.to_owned() == request.path);
    if request_handler.is_some() {
        request_handler.unwrap().2(&mut request, &mut stream);
    } else if request.http_method == "GET".to_owned()
        && fs::metadata(format!("static{}", request.path)).is_ok()
    {
        respond_with_file(&mut stream, request.path);
    } else {
        // TODO: return index.html for path "/"
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

fn respond_with_file(stream: &mut TcpStream, path: String) {
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
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
