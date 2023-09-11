use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::GET => match &req.resource {
                httprequest::Resource::Path(s) => {
                    // parse the url
                    let route: Vec<&str> = s.split("/").collect();
                    match route[i] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_request(stream);
                        }
                        _ => {
                            let resp: HttprResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_request(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_request(stream);
            }
        }
    }
}
