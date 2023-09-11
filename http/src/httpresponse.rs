use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response.body = body;
        response
    }
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }
    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        let h_map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in h_map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}
impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{} Content-Length: {} \r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res1.body.as_ref().unwrap().len(),
            &res1.body()
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("this is a body text".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_text: "OK",
            status_code: "200",
            headers: {
                let mut hash = HashMap::new();
                hash.insert("Content-Type", "text/html");
                Some(hash)
            },
            body: Some("this is a body text".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_http_response_crestion_400() {
        let resposne_actual = HttpResponse::new("404", None, Some("this is a body text".into()));
        let respsonse_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("this is a body text".into()),
        };

        assert_eq!(respsonse_expected, resposne_actual);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", " text/html");
                Some(h)
            },
            body: Some("this is a body text".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual= "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n Content-Length: 19 \r\n\r\nthis is a body text";
        assert_eq!(response_actual, http_string);
    }
}
