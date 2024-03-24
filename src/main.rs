// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ptr::write;
use anyhow::{anyhow};




const HTTP_200: &[u8; 17] = b"HTTP/1.1 200 OK\r\n";
const HTTP_404: &[u8; 24] = b"HTTP/1.1 404 NOT FOUND\r\n";


fn proceed_request(stream: &mut TcpStream) -> anyhow::Result<String> {
    let mut buf = [0; 2048];
    stream.read(&mut buf)?;
    let message = String::from_utf8_lossy(&buf);
    let header = message.lines().next().ok_or(anyhow!("invalid header"))?;
    let path = header.split_whitespace().nth(1).ok_or(anyhow!("invalid path"))?;
    Ok(path.to_string())
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
                match proceed_request(&mut stream)?.as_str() {
                    "/" => write!(stream, "{:?}", HTTP_200)?,
                    echo if echo.starts_with("/echo/") => {
                        let res = echo
                            .split('/')
                            .skip(2)
                            .map(|s|s.as_bytes())
                            .flatten()
                            .copied()
                            .collect::<Vec<_>>();
                        write!(stream, "{:?}", HTTP_200)?;
                        write!(stream, "Content-Type: text/plain \r\n")?;
                        write!(stream, "Content-Length: {}", res.len())?;
                        write!(stream, "{:?}", res)?;
                    },
                    _ =>  write!(stream, "{:?}", HTTP_404)?
                };
                write!(stream, "\r\n")?;
                stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
