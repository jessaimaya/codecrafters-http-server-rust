use crate::httprequest::{self, HttpRequest};
use crate::httpresponse::HttpResponse;
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
                        "echo" => {
                            let resp: HttpResponse = HttpResponse::new(
                                "200",
                                None,
                                Some(route[2].to_string()),
                            );
                            let _ = resp.send_response(stream);

                        },
                        _ => {
                            let resp = HttpResponse::new("404", None, None);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => { // POST, PUT, etc.
                let resp = HttpResponse::new("404", None, None);
                let _ = resp.send_response(stream);
            }
        }
    }
}