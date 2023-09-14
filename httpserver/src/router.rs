use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    // function that takes two parameters a request that is of type HttpRequest and a stream which is a type that impl the Write trait.
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        // steps:
        //     1: check the method
        //     2a: if the path contains some api find service for that api
        //         2b: if there is no api then assume its a static page
        //     3:if there is no GET  method pass a 404 err

        // match the Method of the request (req)
        // HttpRequest {
        //     method = req.method,
        // }
        match req.method {
            // if the Method is of the GET  variant we then check the resource of the req

            // HttpRequest {
            //     method: Method::GET,
            // }
            http::Method::GET => match &req.resource {
                // HttpRequest {
                //     method: httprequest::Method::GET,
                //     resource: httprequest::Resource::Path(path_string),
                // }
                httprequest::Resource::Path(path_string) => {
                    // parse the url
                    let route: Vec<&str> = path_string.split('/').collect();

                    // HttpRequest {
                    //     method: httprequest::Method::GET,
                    //     resource: httprequest::Resource::Path(GET./api/...),
                    // }
                    // after spliting the second [1] element should be the request route
                    match route[1] {
                        // if its "api"
                        "api" => {
                            // invoke the WebServiceHandler function
                            // that is appropriate for the api in the path.
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            // send back the the response to the stream
                            // which impl the Write trait
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            // if there is no "api" being used or located in the path
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                // if the request method is not GET
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
