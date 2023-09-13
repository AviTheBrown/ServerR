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
        contents.ok();
    }
}
