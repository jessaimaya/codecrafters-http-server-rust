use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method{
    Get,
    Post,
    Unitialized,
}

impl From<&str> for Method{
    fn from(m: &str) -> Method {
        match m {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Unitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Unitialized,
}

impl From<&str> for Version {
    fn from(v: &str) -> Version {
        match v {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2" => Version::V2_0,
            _ => Version::Unitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        let mut p_method = Method::Unitialized;
        let mut p_resource = Resource::Path(("".to_string()));
        let mut p_version = Version::Unitialized;
        let mut p_headers: HashMap<String, String> = HashMap::new();
        let mut p_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                p_method = method;
                p_resource = resource;
                p_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                p_headers.insert(key, value);
            } else if line.len() != 0 {
                p_body = line;
            }
        }

        HttpRequest{
            method: p_method,
            version: p_version,
            resource: p_resource,
            headers: p_headers,
            body: p_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (method.into(), Resource::Path(resource.into()), version.into())
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header = s.split(":");
    let mut key = "";
    let mut value = "";

    if let Some(k) = header.next() {
        key = k;
    }
    if let Some(v) = header.next() {
        value = v;
    }

    (key.to_owned(), value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    
        let m: Method = "PUT".into();
        assert_eq!(m, Method::Unitialized);
    }

    #[test]
    fn test_version_into() {
        let v: Version = Version::V1_1.into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn read_http_request() {
        let req_str = String::from("GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let req: HttpRequest = req_str.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(req.headers.len(), 3);
    }
}