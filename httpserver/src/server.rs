use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

// this replicates a Server as all servers have an address on which to connect to it through.
pub struct Server<'a> {
    // liftimes to indicate that the string should be used
    // for the entiter duration of the program.
    socket_addr: &'a str,
}

// this is a non_black box impl of TcpListener::bind()
impl<'a> Server<'a> {
    pub fn new(socker_adr: &'a str) -> Self {
        Server {
            socket_addr: socker_adr,
        }
    }
    pub fn run(&self) {
        // us the socket_addr as the str to connect to bind it to a listener.
        // unwrap is not optimal but use rn for quick iteration.
        // creates a loop to begine listening from the addr(socket.addr)
        // will listen for connections incomming.
        let connection_listener: TcpListener = TcpListener::bind(self.socket_addr).unwrap();
        // stream will take on the value of each incomming connection in turn.
        for stream in connection_listener.incoming() {
            // creates a TCP strean that will listen to the request and responses messages from the server and client
            // once again unwrap is not ideal for error handling. map_err() or or() might be ideal.
            let mut stream = stream.unwrap();
            println!("Connection Established");
            // creates a buffer capacity of 100 bytes that can be transfer
            // at one time on the stream.
            let mut read_buffer = [0; 100];
            // will read the bytes from the buffer
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
