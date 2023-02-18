use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const IP: &str  = "localhost";
const PORT: u16 = 8888;

fn main() {
    let listener = TcpListener::bind(format!("{IP}:{PORT}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

/// Handles a Tcp Connection
///
/// # Arguments
///
/// * `stream` - A stream to handle HTTP 1.1 connection
///
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("web/hello.html").unwrap();
    let length = contents.len();

    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}

