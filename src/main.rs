// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::io::{Read, Write};
use std::net::TcpListener;

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
                let response = b"HTTP/1.1 200 OK\r\n\r\n";
                let mut str = "".to_string();
                stream.read_to_string(&mut str)?;
                stream.write_all(response)?;
                stream.flush()?;
                println!("{str}");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
