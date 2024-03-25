// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use http_server_starter_rust::request::Request;



fn main() -> anyhow::Result<()>{
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let res = Request::parse_request(&mut stream)?;
                let response = res.get_response();
                write!(stream, "{}", response)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
