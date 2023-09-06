pub mod httprequest;
pub mod httpresponse;
use crate::httprequest::{HttpRequest, Resource};

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}
impl From<&str> for Method {
    fn from(meth_type: &str) -> Self {
        match meth_type {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}
impl From<&str> for Version {
    fn from(vers_type: &str) -> Self {
        match vers_type {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        let p: Method = "POST".into();
        assert_eq!(m, Method::GET);
        assert_eq!(p, Method::POST);
    }
    #[test]

    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
    #[test]
    fn test_read_http() {
        let s = String::from(
            "GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n",
        );
        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), "localhost:3000".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
        let req: HttpRequest = s.into();
        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
