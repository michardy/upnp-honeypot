extern crate actix_web;
extern crate http;
use actix_web::{server, App, HttpRequest, HttpResponse, Responder, Result};
use actix_web::middleware::{Middleware, Started, Response};

extern crate quick_xml;

// Device Descriptions

use http::{header, HttpTryFrom};

struct Headers;

impl<S> Middleware<S> for Headers {
	fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
		Ok(Started::Done)
	}

	/// Mathod used to add a server header
	fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse)
		-> Result<Response>
	{
		resp.headers_mut().insert(
			header::HeaderName::try_from("Server").unwrap(),
			header::HeaderValue::from_static("Linux/2.6, UPnP/1.0, miniupnpd/1.0"));
		Ok(Response::Done(resp))
	}
}

fn main() {
	server::new(|| {
		App::new()
			.middleware(Headers)
	})
		.bind("127.0.0.1:8000")
		.expect("Can not bind to port 8000")
		.run();
}
