use std::io::Read;
use std::net::TcpListener;

use crate::httprequest::HttpRequest;
use crate::router::Router;

pub struct Server<'a> {
    socket: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(s: &'a str) -> Self {
        Server {
            socket: s,
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.socket).unwrap();
        println!("Server running on: {}", self.socket);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut read_buffer = [0;1024];

            stream.read(&mut read_buffer).unwrap();

            let req: HttpRequest = String::from_utf8(
                read_buffer.to_vec()
            ).unwrap().into();

            Router::route(req, &mut stream);
        }
    }
}