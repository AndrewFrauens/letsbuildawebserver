use std::net;
use std::io::{BufReader, BufRead, Write};
use std::thread;

/// Echos the content from a TcpStream until it closes.
///
/// # Arguments
///
/// * `stream` - The stream to echo.
fn echo_client(stream: net::TcpStream) -> std::io::Result<()> {
    println!("got a connection {:?}", stream);

    let mut buffer = String::new();
    let mut stream = BufReader::new(stream);

    while !stream.read_line(&mut buffer).is_err(){
        match stream.get_ref().write(buffer.as_bytes()) {
            Ok(_) => buffer.clear(),
            Err(_) => break,
        }
    }

    println!("closing connection to {:?}", stream);

    Ok(())
}

fn main() -> std::io::Result<()> {

    let host = "127.0.0.1";
    let port = 8888;

    let url = format!("{}:{}", host, port);

    println!("The url is: {}\n", url);

    let listener = net::TcpListener::bind(url)?;
    
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || echo_client(stream).expect("failure during run") );
    }

    Ok(())
}
