use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum Body {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    pub version: &'a str,
    pub status_code: &'a str,
    pub status_text: &'a str,
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub body: Option<Body>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        HttpResponse {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<Body>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code.into();
        }

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/plain");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "201" => "Created".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not found".into(),
        };

        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        let real_body = match &self.body {
            Some(b) => match b {
                Body::Text(t) => t.as_bytes(),
                Body::Binary(b) => b.as_slice(),
            },
            None => &[],
        };
        write_stream.write_all(&real_body)?;
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut headers = String::from("");

        for (k, v) in map.iter() {
            headers = format!("{}{}:{}\r\n", headers, k, v)
        }
        headers
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let result = res.clone();
        let body_len = match &result.body {
            Some(b) => match b {
                Body::Text(t) => t.len(),
                Body::Binary(b) => b.len(),
            },
            None => 0,
        };
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n",
            &result.version(),
            &result.status_code(),
            &result.status_text(),
            &result.headers(),
            &body_len,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_200() {
        let current_response =
            HttpResponse::new("200", None, Some(Body::Text(("My body content").into())));
        let expected_response = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some(Body::Text(("My body content").into())),
        };

        assert_eq!(current_response, expected_response);
    }
}
