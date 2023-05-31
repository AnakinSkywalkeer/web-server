use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();
    let response = format!("HTTP/1.1 200 OK{}\r\n\r\n{}", contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
