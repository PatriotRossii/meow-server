use std::collections::HashMap;

#[derive(Debug)]
pub struct StatusCode {
	pub code: i16,
	pub description: String,
}

impl ToString for StatusCode {
	fn to_string(&self) -> String {
		format!("{} {}", self.code.to_string(), self.description)
	}
}

#[derive(Debug)]
pub struct ResponseHead {
	http_version: String,
	status_code: StatusCode,	
}

impl ToString for ResponseHead {
	fn to_string(&self) -> String {
		format!("{} {}", self.http_version, self.status_code.to_string())
	}	
}

#[derive(Debug)]
pub struct Headers {
	headers: HashMap<String, String>,
}

impl Headers {
	pub fn new() -> Self {
		Self {
			headers: HashMap::new(),
		}
	}

	pub fn set(mut self, name: String, value: String) -> Self {
		self.headers.insert(name, value);
		self
	}

	pub fn get(&self, name: &String) -> Option<&String> {
		self.headers.get(name)
	}
}

impl ToString for Headers {
	fn to_string(&self) -> String {
		self.headers.iter().map(
			|(ref key, ref value)| {
				format!("{}: {}\r\n", key, value)
			}
		).collect()
	}
}

#[derive(Debug)]
pub struct Response {
	head: ResponseHead,
	headers: Headers,
	body: String,
}

impl ToString for Response {
	fn to_string(&self) -> String {
		format!("{}\r\n{}\r\n{}", self.head.to_string(), self.headers.to_string(), self.body)
	}
}

impl Response {
	pub(crate) fn new() -> Response {
		Response {
			head: ResponseHead {
				http_version: String::from("HTTP/1.1"),
				status_code: StatusCode {
					code: 200,
					description: String::from("Ok"),
				}
			},
			headers: Headers::new()
				.set("Content-Type".to_string(), "text/html; charset=utf-8".to_string())
				.set("Content-Length".to_string(), "0".to_string()),
			body: String::new(),
		}
	}

	pub(crate) fn body(mut self, body: String) -> Response {
		self.headers = self.headers.set("Content-Length".to_string(), body.len().to_string());
		self.body = body;

		self
	}

	pub(crate) fn status_code(mut self, status_code: StatusCode) -> Response {
		self.head.status_code = status_code;
		self
	}

	pub(crate) fn headers(mut self, headers: Headers) -> Response {
		self.headers = headers;
		self

	}
}