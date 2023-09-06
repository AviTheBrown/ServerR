use crate::{Method, Version};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

// fn process_req_line(s: &str) -> (Method, Resource, Version) {}
// fn process_header_line(s: &str) -> (String, String) {}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = String::new();

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resorce, version) = parsed_req_line(line);
                parsed_method = method;
                parsed_resource = resorce;
                parsed_version = version;
            } else if line.contains(':') {
                let (key, value) = parsed_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.is_empty() {
            } else {
                parsed_msg_body = line.to_string();
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}

fn parsed_req_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn parsed_header_line(line: &str) -> (String, String) {
    let (key, value) = line.split_once(':').unwrap();
    (key.to_string(), value.to_string())
}
