use crate::server::Server;

pub mod server;

#[tokio::main]
async fn main() {
	let server = Server::new();
	server.start("127.0.0.1:80").await.unwrap();
}
