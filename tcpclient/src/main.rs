use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    // writes to the stream: convets the string to bytes.
    stream.write("Hello".as_bytes()).unwrap();
    // creates a buffer that is able to hold 5 bytes
    let mut buffer = [0; 5];
    // reads the buffer and muts it
    stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server: {:?}",
        // converts from bytes (u8) to string type
        str::from_utf8(&buffer).unwrap()
    );
}
