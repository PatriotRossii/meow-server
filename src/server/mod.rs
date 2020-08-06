use tokio::net::{TcpListener, TcpStream};

pub struct Server { }

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
		Server{}
	}

	pub async fn start<A>(&self, addr: A) -> Result<(), ServerError>
	where
		A: tokio::net::ToSocketAddrs + std::fmt::Display + Copy
	{
		let mut listener = TcpListener::bind(addr).await?;

		println!("Start listening on {}", addr);

		loop {
			let (socket, ip) = listener.accept().await?;
			println!("Established connection with {}", ip);
			self.process(socket).await;
		}
	}

	pub(crate) async fn process(&self, _socket: TcpStream) {
		todo!()
	}
}