mod request;
mod response;

use crate::server::response::StatusCode;
use std::{collections::HashMap, path::Path};

use crate::server::request::Request;
use crate::server::response::Response;


use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::fs::{read_dir, File};

pub struct StaticStorage{
	map: HashMap<String, Option<String>>,
}

impl StaticStorage {
	pub(crate) fn new() -> StaticStorage {
		StaticStorage{
			map: HashMap::new(),
		}
	}
	pub(crate) async fn initialize(&mut self, path: &str) -> Result<(), ServerError> {
		let mut dir_stream = read_dir(Path::new(path)).await?;
		while let Some(entry) = dir_stream.next_entry().await? {
			if let Ok(file_type) = entry.file_type().await {
				if file_type.is_file() {
					match entry.path().extension() {
						Some(ext) => {
							let extension = ext.to_str().unwrap_or("");
							match extension {
								"htm" | "html" => {
									let mut file_path = entry.path().to_str().unwrap().to_string();
									let mut buffer = String::new();

									let mut f = File::open(file_path.clone()).await?;
									f.read_to_string(&mut buffer).await?;

									let mut path = path.to_string();
									path.push('/');

									file_path = file_path.strip_prefix(&path).unwrap().to_string();

									self.map.insert(file_path, Some(buffer));
								}
								_ => {}
							}
						}
						_ => {}
					}
				}
			}
		}
		Ok(())
	}

	pub(crate) async fn get_entry(&mut self, key: &str) -> Result<&Option<String>, ()> {
		if let Some(x) = self.map.get(&key.to_string()) {
			Ok(x)
		} else {
			Err(())
		}
	}
}

pub struct Server {
	pub(crate) storage: StaticStorage,
}


#[derive(Debug)]
pub enum ServerError {
	Io(std::io::Error),
}

impl From<std::io::Error> for ServerError {
	fn from(error: std::io::Error) -> ServerError {
		ServerError::Io(error)
	}
}

impl Server {
	pub fn new() -> Server {
		Server{
			storage: StaticStorage::new(),
		}
	}

	pub async fn start<A>(&mut self, addr: A) -> Result<(), ServerError>
	where
		A: tokio::net::ToSocketAddrs + std::fmt::Display + Copy
	{
		let mut listener = TcpListener::bind(addr).await?;
		self.storage.initialize("static").await?;

		println!("Start listening on {}", addr);

		loop {
			let (socket, ip) = listener.accept().await?;
			println!("Established connection with {}", ip);
			self.process(socket).await;
		}
	}


	pub(crate) async fn process(&mut self, mut socket: TcpStream) {
		let mut buffer = [0u8; 2048];
		socket.read(&mut buffer).await.unwrap();

		let request = Request::from(&mut buffer);

		let data = self.handle_request(&request).await.to_string();
		println!("{}", data);

		socket.write(data.as_bytes()).await.unwrap();
	}

	pub(crate) async fn handle_request(&mut self, request: &Request) -> Response {
		let mut path = request.get_uri();
		if path == "/" {
			path = "index.html"; 
		} else {
			path = match path.strip_prefix("/") {
				Some(path) => path,
				None => {
					return Response::new()
						.body(String::new())
						.status_code(StatusCode {
							code: 400,
							description: String::from("Bad Request"),
						});
				} 
			}
		}

		if let Ok(entry) = self.storage.get_entry(path).await {
			Response::new()
				.body(entry.as_ref().unwrap().to_string())
				.status_code(StatusCode{
					code: 200,
					description: String::from("OK"),
				})
		} else {
			Response::new()
				.body(String::new())
				.status_code(StatusCode{
					code: 404,
					description: String::from("Not Found"),
				})
		}
	}
}