use std::{env, os::unix::net::UnixStream, error::Error};
use replayd::{ipc::{message::{self, Message, Response}}, socket_path};

fn main() -> Result<(), Box<dyn Error>>{
	// Look for commands
	
	const SEP: &str = " ";
	// Fast string iterator folding, credit to mdonoughe 
	// https://stackoverflow.com/questions/56033289/join-iterator-of-str#comment115603566_56033952
	let mut ipc_writebuf = env::args().skip(1).fold(String::new(), |mut a, b| {
		a.reserve(b.len() + 1);
		a.push_str(&b);
		a.push_str(SEP);
		a
	});
	ipc_writebuf.pop(); // Remove trailing space
	let mut ipc_readbuf: [u8; message::BUF_SIZE] = [0; message::BUF_SIZE];

	// Send command to socket
	let runtime_dir = replayd::runtime_dir();
	let socket_path = replayd::socket_path(&runtime_dir);
	let mut stream = UnixStream::connect(&socket_path).map_err(|err| {
		eprintln!("Failed to connect to socket at {}", &socket_path);
		err
	})?;

	let message = Message::from(ipc_writebuf)?;
	message.encode(&mut stream)?;

	// Wait for response
	let resp = Response::decode(&mut stream, &mut ipc_readbuf)?;
	println!("{}", resp.body());
	resp.check().map_err(|err| err.into())
}


