use std::env;
use crate::httprequest::{self, HttpRequest};
use crate::httpresponse::HttpResponse;
use std::collections::HashMap;
use std::io::Write;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "" | "/" => {
                            let resp: HttpResponse = HttpResponse::new("200", None, None);
                            let _ = resp.send_response(stream);
                        },
                        "user-agent" => {
                            let mut resp: HttpResponse = HttpResponse::new("200", None, None);
                            if let Some(header) = req.headers.get("User-Agent") {
                                resp.body = Some(header.trim().to_string());
                            }
                            let _ = resp.send_response(stream);
                        },
                        "files" => {
                            let filename = route[2];
                            let args: Vec<String> = env::args().collect();
                            let dir = args.last().unwrap().to_string();
                            let exists = std::path::Path::new(&format!("{}{}", dir, filename)).exists();
                            if exists {
                                let b= std::fs::read_to_string(&format!("{}{}", dir, filename)).unwrap();
                                let b_len = b.len().to_string();
                                let h = Some(HashMap::from([
                                    ("Content-Type","application/octet-stream"),
                                    ("Content-Length", &b_len)
                                ]));
                                let resp: HttpResponse = HttpResponse::new("200", h, Some(b));
                                let _ = resp.send_response(stream);
                            } else {
                                let resp: HttpResponse = HttpResponse::new("404", None, None);
                                let _ = resp.send_response(stream);
                            }
                        },
                        "echo" => {
                            let req_headers = req.headers.clone();
                            let accept_encoding = req_headers.get("Accept-Encoding"); 
                            let mut resp: HttpResponse = HttpResponse::new(
                                "200",
                                None,
                                Some(route[2].to_string()),
                            );
                            if accept_encoding.is_some() {
                                let mut encodings = accept_encoding.unwrap().split(",");
                                if let Some(h) = encodings.find(|e| e.to_lowercase().trim() == "gzip") {
                                    resp.headers = Some(HashMap::from([
                                        ("Content-Encoding", h as &str)
                                    ]));
                                }
                            }
                            let _ = resp.send_response(stream);

                        },
                        _ => {
                            let resp = HttpResponse::new("404", None, None);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            httprequest::Method::Post => match &req.resource {
                httprequest::Resource::Path(s) => {
                let route: Vec<&str> = s.split("/").collect();
                match route[1] {
                    "files" => {
                        let filename = route[2];
                        let args: Vec<String> = env::args().collect();
                        let dir = args.last().unwrap().to_string();
                        let data = req.body;

                        let mut file = std::fs::File::create(&format!("{}{}", dir, filename)).unwrap();
                        let _ = file.write_all(data.trim_end_matches('\0').as_bytes());
                        let resp = HttpResponse::new("201", None, None);
                        let _ = resp.send_response(stream);
                    },
                    _ => {
                        let resp = HttpResponse::new("404", None, None);
                        let _ = resp.send_response(stream);
                    }
                }
            }
            },
            _ => { // PUT, etc.
                let resp = HttpResponse::new("404", None, None);
                let _ = resp.send_response(stream);
            }
        }
    }
}