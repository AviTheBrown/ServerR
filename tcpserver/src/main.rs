use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // creates a server
    let server = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("RUNNING ON PORT 3000");

    // start the stream connection
    for stream in server.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection made");

        let mut buffer = [0; 0124];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
