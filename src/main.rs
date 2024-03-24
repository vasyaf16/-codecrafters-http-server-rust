// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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
                println!("{:?}",&stream);
                println!("accepted new connection");
                let response = match proceed_request(&mut stream)?.as_str() {
                    "/" => {
                        let mut res = HTTP_200.to_vec();
                        res.extend(b"\r\n");
                        res
                    },
                    echo if echo.starts_with("/echo/") => {
                        let res = echo
                            .splitn(3, "/")
                            .skip(2)
                            .flat_map(|s| s.bytes())
                            .collect::<Vec<_>>();
                        println!("{:?}", res);
                        let mut response = HTTP_200.to_vec();
                        response.extend(b"Content-Type: text/plain\r\n");
                        response.extend((format!("Content-Length: {}\r\n\r\n", res.len())).as_bytes());
                        response.extend(res);
                        response
                    },
                    _ => {
                        let mut res = HTTP_404.to_vec();
                        res.extend(b"\r\n");
                        res
                    }
                };
                stream.write_all(response.as_slice())?;
                stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
