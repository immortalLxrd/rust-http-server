pub struct ResponseHeaders<'a> {
    content_type: &'a str,
    content_length: usize,
}

impl<'a> ResponseHeaders<'a> {
    pub fn new(content_type: &'a str, content: &'a str) -> Self {
        let content_length = content.len();

        Self {
            content_type,
            content_length,
        }
    }
}

pub struct ResponseMessage<'a> {
    protocol: &'a str,
    status_code: &'a str,
    message: &'a str,
    headers: Option<ResponseHeaders<'a>>,
    body: Option<&'a str>,
}

impl<'a> ResponseMessage<'a> {
    pub fn new(
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

    pub fn as_bytes(self) -> Box<[u8]> {
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

        result.as_bytes().to_vec().into_boxed_slice()
    }
}
