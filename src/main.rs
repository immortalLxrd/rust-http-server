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

struct ResponseHeaders<'a> {
    content_type: &'a str,
    content_length: usize,
}

struct ResponseMessage<'a> {
    protocol: &'a str,
    status_code: &'a str,
    message: &'a str,
    headers: Option<ResponseHeaders<'a>>,
    body: Option<&'a str>,
}

impl<'a> ResponseMessage<'a> {
    fn new(
        protocol: &'a str,
        status_code: &'a str,
        message: &'a str,
        headers: Option<ResponseHeaders<'a>>,
        body: Option<&'a str>,
    ) -> Self {
        Self {
            protocol,
            status_code,
            message,
            headers,
            body,
        }
    }

    fn to_string(self) -> String {
        let mut result = format!(
            "{} {} {}\r\n",
            self.protocol, self.status_code, self.message
        );
        if let (Some(headers), Some(body)) = (self.headers, self.body) {
            result += &format!(
                "Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                headers.content_type, headers.content_length, body,
            );
        }
        result
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; 128] = [0; 128];
    stream.read(&mut buf)?;

    let content = str::from_utf8(&buf)?;
    let content_splited: Vec<&str> = content.split(" ").collect();
    let path = content_splited[1].split("/").collect::<Vec<&str>>();

    match path {
        path if path[1] == "" => {
            let message = ResponseMessage::new("HTTP/1.1", "200", "OK", Option::None, Option::None)
                .to_string();
            stream.write(message.as_bytes())?;
        }
        path if path[1] == "echo" => {
            let headers = ResponseHeaders {
                content_type: "text/plain",
                content_length: path[2].len(),
            };
            let message = ResponseMessage::new(
                "HTTP/1.1",
                "200",
                "OK",
                Option::Some(headers),
                Option::Some(path[2]),
            )
            .to_string();
            stream.write(message.as_bytes())?;
        }
        _ => {
            let message =
                ResponseMessage::new("HTTP/1.1", "404", "Not Found", Option::None, Option::None)
                    .to_string();
            stream.write(message.as_bytes())?;
        }
    }

    Ok(())
}
