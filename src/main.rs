use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        handle_connection(stream?)
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    stream.read(&mut buf).unwrap();

    let request = String::from_utf8_lossy(&buf[..]).to_string();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
        request.len(),
        request
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
