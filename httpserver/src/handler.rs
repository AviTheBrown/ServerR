use super::handler::{Handler, PageNotFound, StaticPageHandler, WebServerHandler};
use std::io::prelude::*;
use http::{Request, Response};

pub struct Router; 

impl Router {
	pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
		match req.method {
			"GET" => 
		}
	}
}