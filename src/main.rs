use mysql::prelude::*;
use mysql::*;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    name: Option<String>,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // TODO: add multi threading: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
        handle_connection(stream);
    }
}

// TODO: rename function
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();

    let http_request: Vec<_> = lines
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    if request_line == "GET /articles HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let mut contents = fs::read_to_string("index.html").unwrap();

        let result = request_database();
        println!("Result: {:?}", result);
        if result.is_ok() {
            let users = result
                .unwrap()
                .iter()
                .map(|user| format!("<li>{}</li>", user.name.as_ref().unwrap()))
                .collect::<Vec<String>>()
                .join("\r\n");
            contents = contents.replace("#_Users", &users);
        } else {
            contents = contents.replace("#_Users", "<li>ERROR</li>");
        }

        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn request_database() -> std::result::Result<Vec<User>, Box<dyn std::error::Error>> {
    println!("Request database...");
    let url = "mysql://root:root@localhost:3306/rust-test";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    let users = conn.query_map("SELECT id, name from user", |(id, name)| User { id, name })?;

    println!("Users: {:?}", users);

    return Ok(users);
}
