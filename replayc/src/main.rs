use std::{env, os::unix::net::UnixStream, error::Error};
use replayd::ipc::{socket, message};

fn main() -> Result<(), Box<dyn Error>>{
	// Look for commands
	
	const SEP: &str = " ";
	// Fast string iterator folding, credit to mdonoughe 
	// https://stackoverflow.com/questions/56033289/join-iterator-of-str#comment115603566_56033952
	let ipc_buf = env::args().skip(1).fold(String::new(), |mut a, b| {
		a.reserve(b.len() + 1);
		a.push_str(&b);
		a.push_str(SEP);
		a
	});
	let ipc_writebuf = ipc_buf.trim_end();
	let mut ipc_readbuf = socket::ReadBuffer::new();

	// Send command to socket
	let socket_path = socket::get_socket_path(replayd::runtime_dir().as_str());
	let mut stream = UnixStream::connect(&socket_path).map_err(|err| {
		eprintln!("Failed to connect to socket at {}", &socket_path);
		err
	})?;

	socket::write_msg_sync(&mut stream, ipc_writebuf)?;
	let (status, msg) = ipc_readbuf.read_resp(&mut stream)?;
	println!("{}", msg);

	Ok(())
}


