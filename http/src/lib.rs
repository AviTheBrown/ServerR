pub mod httprequest;

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }
}
