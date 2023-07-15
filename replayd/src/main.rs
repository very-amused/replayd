use std::error::Error;
use std::mem::size_of;

use ipc::header::Header;
use tokio::io::Interest;
use tokio::net::UnixStream;

mod ipc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Connecting to Unix Socket");
		let stream = ipc::socket::connect().await?;

		// Read and write messages
		loop {
			// Wait for socket to be ready for r/w
			let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;

			if ready.is_readable() {
				let msg = read_msg(&stream).await?;
				println!("{}", msg);
				break;
			}
		}
		Ok(())
}

async fn read_msg(stream: &UnixStream) -> Result<String, tokio::io::Error> {
	// Read message header
	let mut header_buf = vec![0; ipc::header::HEADER_SIZE];
	match stream.try_read(header_buf) {
		Ok(_) => {
			let mut id: u64;
			let mut 
		}
	}


}
