use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
	verson: &'a str,
	status_code: &'s str,
	status_text: &'a str,
	headers: Option<HashMap<&'a str, &'a str>,
	body: Option<String>,
}
