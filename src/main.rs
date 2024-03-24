// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use anyhow::{anyhow, Error};




const HTTP_200: &[u8; 19] = b"HTTP/1.1 200 OK\r\n\r\n";
const HTTP_404: &[u8; 26] = b"HTTP/1.1 404 NOT FOUND\r\n\r\n";


fn proceed_request(stream: &mut TcpStream) -> anyhow::Result<&str> {
    let mut buf = [0; 2048];
    stream.read(&mut buf)?;
    let message = String::from_utf8_lossy(&buf);
    let header = message.lines().next().ok_or(anyhow!("invalid header"))?;
    let path = header.split_whitespace().nth(1).ok_or(anyhow!("invalid path"))?;
    Ok(path)
}

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
                let response = match proceed_request(&mut stream)? {
                    "/" => HTTP_200.as_slice(),
                    _ => HTTP_404.as_slice()
                };
                stream.write_all(response)?;
                stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
