use std::{
    env,
    error::Error,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    str, thread,
};

mod response;
use response::response::{ResponseHeaders, ResponseMessage};

fn handle_connection(mut stream: TcpStream, dir: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut buf: [u8; 4096] = [0; 4096];
    stream.read(&mut buf)?;

    let content = str::from_utf8(&buf)?;
    let content_splited: Vec<&str> = content.split(' ').collect();

    let method = content_splited[0];
    let path = content_splited[1].strip_prefix('/').unwrap();
    let (route, body) = if let Some((route, body)) = path.split_once('/') {
        (route, body)
    } else {
        (path, "")
    };

    let (_, headers) = content.split_once("\r\n").unwrap();
    let headers_splited: Vec<&str> = headers.split("\r\n").collect();

    let not_found =
        ResponseMessage::new("HTTP/1.1", "404", "Not Found", Option::None, Option::None).as_bytes();

    let response = match route {
        "" => ResponseMessage::new("HTTP/1.1", "200", "OK", Option::None, Option::None).as_bytes(),
        "echo" if !body.is_empty() => {
            let headers = ResponseHeaders::new("text/plain", body);
            ResponseMessage::new(
                "HTTP/1.1",
                "200",
                "OK",
                Option::Some(headers),
                Option::Some(body),
            )
            .as_bytes()
        }
        "user-agent" => {
            let user_agent_pattern = "User-Agent: ";
            let finded_header = headers_splited
                .iter()
                .find(|item| item.starts_with(user_agent_pattern))
                .unwrap();
            let response_content = finded_header.strip_prefix(user_agent_pattern).unwrap();

            let headers = ResponseHeaders::new("text/plain", response_content);
            ResponseMessage::new(
                "HTTP/1.1",
                "200",
                "OK",
                Option::Some(headers),
                Option::Some(response_content),
            )
            .as_bytes()
        }
        "files" if !body.is_empty() => {
            if let Some(dir) = dir {
                let dir = if let Some(dir) = dir.strip_suffix("/") {
                    dir
                } else {
                    &dir
                };
                let s = String::from(dir) + &"/" + body;
                let path = Path::new(&s);

                match method {
                    "GET" => {
                        if path.is_file() {
                            match fs::read_to_string(path) {
                                Ok(content) => {
                                    let headers =
                                        ResponseHeaders::new("application/octet-stream", &content);
                                    ResponseMessage::new(
                                        "HTTP/1.1",
                                        "200",
                                        "OK",
                                        Option::Some(headers),
                                        Option::Some(&content),
                                    )
                                    .as_bytes()
                                }
                                Err(_) => not_found,
                            }
                        } else {
                            not_found
                        }
                    }
                    "POST" => {
                        not_found
                    }
                    _ => not_found,
                }
            } else {
                not_found
            }
        }
        _ => not_found,
    };
    stream.write(&response)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 2 && args[1] == "--directory" && !args[2].is_empty() {
        Option::Some(args[2].clone())
    } else {
        Option::None
    };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let dir = dir.clone();

        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_connection(stream, dir).unwrap());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
