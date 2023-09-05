use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    const MESSAGE: &str = "Hello Server";

    let mut buffer = [0; MESSAGE.len()];
    stream.write(MESSAGE.as_bytes()).unwrap();
    stream.read(&mut buffer).unwrap();

    println!(
        "Message from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    )
}
