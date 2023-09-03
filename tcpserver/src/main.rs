use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // creates a server
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    // continues to execute the server by loop
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        // creates a bufffer that can hold up to 1024 bytes
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
