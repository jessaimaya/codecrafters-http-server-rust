pub mod httprequest;
pub mod httpresponse;
pub mod server;
pub mod router;

use server::Server;



fn main() {
    println!("Logs from your program will appear here!");

    let server = Server::new("127.0.0.1:4221");
    server.run();

    /*let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut incomming_buffer = [0; 1024];
                println!("accepted new connection: {}", std::str::from_utf8(&incomming_buffer).unwrap());

                let _ = stream.read(&mut incomming_buffer).unwrap();
                let request_str = std::str::from_utf8(&incomming_buffer).unwrap();
                let req: HttpRequest = String::from(request_str).into();
                let mut res = HttpResponse::default();

                if req.resource == Resource::Path("/".into()) {
                    res = HttpResponse::new("200", None, None);
                } else {
                    res = HttpResponse::new("404", None, None);
                }

                let _ = res.send_response(&mut stream);

                // let buf = "HTTP/1.1 200 OK\r\n\r\n";
                // let _ = stream.write(outcomming_buf.as_bytes());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    */
}
