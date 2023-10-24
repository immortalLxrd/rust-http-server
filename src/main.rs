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

impl<'a> ResponseHeaders<'a> {
    fn new(content_type: &'a str, content: &'a str) -> Self {
        let content_length = content.len();

        Self {
            content_type,
            content_length,
        }
    }
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
            let headers = ResponseHeaders::new(headers.content_type, body);
            result += &format!(
                "Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                headers.content_type, headers.content_length, body,
            );
        } else {
            result += "\r\n";
        }
        result
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; 128] = [0; 128];
    stream.read(&mut buf)?;

    let content = str::from_utf8(&buf)?;
    let content_splited: Vec<&str> = content.split(' ').collect();
    let path = content_splited[1].strip_prefix('/').unwrap();
    let (route, body) = if let Some((route, body)) = path.split_once('/') {
        (route, body)
    } else {
        (path, "")
    };

    match (route, body) {
        (route, _) if route == "" => {
            let message = ResponseMessage::new("HTTP/1.1", "200", "OK", Option::None, Option::None)
                .to_string();
            stream.write(message.as_bytes())?;
        }
        (route, body) if route == "echo" && body.len() > 0 => {
            let headers = ResponseHeaders::new("text/plain", body);
            let message = ResponseMessage::new(
                "HTTP/1.1",
                "200",
                "OK",
                Option::Some(headers),
                Option::Some(body),
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
