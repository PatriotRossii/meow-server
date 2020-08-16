use itertools::Itertools;

#[derive(Clone, Debug)]
pub(crate) struct RequestHead {
	method: Method,
	uri: String,
	http_version: String,
}

struct Headers {}

#[derive(Clone, Debug)]
pub(crate) struct Request {
	head: RequestHead,
}

impl Request {
	pub fn get_method(&self) -> Method {
		return self.head.method
	}
	pub fn get_uri(&self) -> &str {
		return &self.head.uri
	}
	pub fn get_http_version(&self) -> &str {
		return &self.head.http_version
	}
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Method {
	OPTIONS,
	GET,
	HEAD,
	POST,
	PUT,
	PATCH,
	DELETE,
	TRACE,
	CONNECT,
}

impl From<&str> for Method {
	fn from(method: &str) -> Method {
		match method {
			"OPTIONS" => Self::OPTIONS,
			"GET" => Self::GET,
			"HEAD" => Self::HEAD,
			"POST" => Self::POST,
			"PUT" => Self::PUT,
			"PATCH" => Self::PATCH,
			"DELETE" => Self::DELETE,
			"TRACE" => Self::TRACE,
			"CONNECT" => Self::CONNECT,
			_ => panic!()
		}
	}
}

pub(crate) fn parse_head(header: &[u8]) -> RequestHead {
	let header = std::str::from_utf8(header).unwrap();

	let (header, _) = header.split_at(
		header.find('\n').unwrap()
	);
	header.split(' ').collect_tuple().map(
		|(method, uri, http_version)| RequestHead { method: method.into(), uri: uri.to_string(), http_version: http_version.to_string() }
	).unwrap()
}

impl From<&mut [u8; 2048]> for Request {
	fn from(bytes: &mut [u8; 2048]) -> Request {
		Request {
			head: parse_head(bytes),
		}
	}
}