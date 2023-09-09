use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socker_adr: &'a str) -> Self {
        Server {
            socket_addr: socker_adr,
        }
    }
    pub fn run(&self) {
        let connection_listener: TcpListener = TcpListener::bind(self.socket_addr).unwrap();
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection Established");
            let mut read_buffer = [0; 100];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
