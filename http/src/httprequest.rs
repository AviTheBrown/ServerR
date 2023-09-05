use std::collections::HashMap;
use crate::{httprequest, Method, Version}

#[derive(Debug)]
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

impl From<&str> for HttpRequest {
    fn from(req: &str) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource =Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_message = "";

        for line in req.lines() {
            // if the line is of type request line invoke fn process_req_line()
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line)
                parsed_message = method;
                parsed_resource = resource;
                parsed_version = version;
            } // if the line is a header line invoke fn proces_header_line()
            else if line.contains(":") {
                let (key , value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } // if line contains a black line then do nothing
            else if line.len() == 0 {}
            // if none of these treat as message body 
            else {
                parsed_message = line;
            }
            HttpRequest {
                method: parsed_method,
                version: parsed_version,
                resource: parsed_resource,
                headers: parsed_headers,
                msg_body: parsed_message.to_string();
            }
        }
}
