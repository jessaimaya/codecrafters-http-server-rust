use std::borrow::BorrowMut;
use std::{io::Write, net::TcpListener};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming().borrow_mut() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let buf = "HTTP/1.1 200 OK\r\n\r\n";
                let _ = stream.write(buf.as_bytes());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
