use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Logs from your program will appear here!");

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
    let mut buf: [u8; 128] = [0; 128];
    stream.read(&mut buf)?;

    let content = str::from_utf8(&buf)?;
    let content_splited: Vec<&str> = content.split(" ").collect();
    let path = content_splited[1];

    match path {
        "/" => {
            stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
        }
        path => {
            stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
        }
    }

    Ok(())
}
