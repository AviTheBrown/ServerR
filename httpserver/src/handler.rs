use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default;
use std::env;
use std::fs;

pub trait Handler {
    // will take a reference to a HtttpRequest and return an instance of a response (HttpResponse)
    fn handle(req: &HttpRequest) -> HttpResponse;
    // will take a file name as reference and return an Option
    fn load_file(file_name: &str) -> Option<String> {
        // CARGO_MANIFEST_DIR points to the dir path that is using it.
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        // PUBLIC_PATH points to the path if it is set if not the unwrap_or() will set to a fall back value (default_path)
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        // displays the entire path to the std io
        let full_path = format!("{}/{}", public_path, file_name);

        // will read the entire file and convert it to a string.
        let contents = fs::read_to_string(full_path);
        // will then convert the Result Ok() value of fs::read_to_string into a Some() value from an Option
        // which is our return type.
        contents.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}
pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

impl Handler for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        // Parse the Url
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            // this is used as a catch all for any route that is not defined in teh first 2 match arms.
            path => match Self::load_file(path) {
                // constructs a new HttpResponse instance
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/html");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    // return the new instance of the response.
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                // if unable to create a new httprepsonse instsance then construct it with
                // a 404 status_code
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}
