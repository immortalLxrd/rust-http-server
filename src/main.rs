use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    str, thread,
};

mod response;
use response::response::{ResponseHeaders, ResponseMessage};

fn get_file_content(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    Ok(buf)
}

fn handle_connection(mut stream: TcpStream, dir: Option<String>) -> Result<(), Box<dyn Error>> {
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

    let (_, headers) = content.split_once("\r\n").unwrap();
    let headers_splited: Vec<&str> = headers.split("\r\n").collect();

    let not_found =
        &ResponseMessage::new("HTTP/1.1", "404", "Not Found", Option::None, Option::None)
            .as_bytes();

    match route {
        "" => {
            let message =
                &ResponseMessage::new("HTTP/1.1", "200", "OK", Option::None, Option::None)
                    .as_bytes();
            stream.write(message)?;
        }
        "echo" if !body.is_empty() => {
            let headers = ResponseHeaders::new("text/plain", body);
            let message = &ResponseMessage::new(
                "HTTP/1.1",
                "200",
                "OK",
                Option::Some(headers),
                Option::Some(body),
            )
            .as_bytes();
            stream.write(message)?;
        }
        "user-agent" => {
            let user_agent_pattern = "User-Agent: ";
            let finded_header = headers_splited
                .iter()
                .find(|item| item.starts_with(user_agent_pattern))
                .unwrap();
            let response_content = finded_header.strip_prefix(user_agent_pattern).unwrap();

            let headers = ResponseHeaders::new("text/plain", response_content);
            let message = &ResponseMessage::new(
                "HTTP/1.1",
                "200",
                "OK",
                Option::Some(headers),
                Option::Some(response_content),
            )
            .as_bytes();
            stream.write(message)?;
        }
        "files" if !body.is_empty() => {
            if let Some(dir) = dir {
                let dir = if let Some(dir) = dir.strip_suffix("/") {
                    dir
                } else {
                    &dir
                };
                let s = dir.to_owned() + &"/" + body;
                let path = Path::new(&s);

                match get_file_content(path) {
                    Ok(content) => {
                        let headers = ResponseHeaders::new("application/octet-stream", &content);
                        let message = &ResponseMessage::new(
                            "HTTP/1.1",
                            "200",
                            "OK",
                            Option::Some(headers),
                            Option::Some(&content),
                        )
                        .as_bytes();
                        stream.write(message)?;

                        return Ok(());
                    }
                    Err(_) => (),
                }
            }
            stream.write(not_found)?;
        }
        _ => {
            stream.write(not_found)?;
        }
    }

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
