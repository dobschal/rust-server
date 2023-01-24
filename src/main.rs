pub mod database;
pub mod entities;
pub mod request_handler;

use crate::request_handler::article::get;
use database::database::connect;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let _ = connect();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // TODO: add multi threading: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Incoming Request: {:#?}", http_request);
    let (http_method, path) = http_request[0].split_at(http_request[0].find(" ").unwrap());
    if http_method == "GET" && path.trim().starts_with("/articles") {
        get(&mut stream);
    } else {
        repond_with_error(&mut stream);
    }
}

fn repond_with_error(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
