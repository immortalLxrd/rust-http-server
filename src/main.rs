// Uncomment this block to pass the first stage
use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    let mut buf = [0; 128];
    stream.read(&mut buf)?;
    stream.write(response)?;
    Ok(())
}
