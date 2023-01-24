pub mod article {
    use std::{fs, io::Write, net::TcpStream};

    use crate::database::database;

    pub fn get(stream: &mut TcpStream) {
        let status_line = "HTTP/1.1 200 OK";
        let mut contents = fs::read_to_string("index.html").unwrap();

        let result = database::request_database();
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
    }
}