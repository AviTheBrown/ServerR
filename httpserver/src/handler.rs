use http::{http::HttpRequest, http::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String>;
}
