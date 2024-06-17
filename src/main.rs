pub mod httprequest;
pub mod httpresponse;
pub mod router;
pub mod server;

use server::Server;

fn main() {
    println!("Logs from your program will appear here!");

    let server = Server::new("127.0.0.1:4221");
    server.run();
}
