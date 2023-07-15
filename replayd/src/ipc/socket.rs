use std::{io, u16};

use tokio::{net::UnixStream, io::{AsyncReadExt,AsyncWriteExt}};

pub fn get_socket_path(runtime_dir: &str) -> String {
	const SOCKET_NAME: &str = "replayd.sock";
	format!("{}/{}", runtime_dir, SOCKET_NAME)
}

	
// Size of reusable message buffer/Max message size
pub const BUF_SIZE: usize = 8192;

/// A reusable read buffer for IPC messages
pub struct ReadBuffer {
	buf: [u8; BUF_SIZE]
}

impl ReadBuffer {
	// Allocate a new ReadBuffer
	pub fn new() -> Self {
		ReadBuffer { buf: [0; BUF_SIZE] }
	}

	// Read an IPC message
	pub async fn read_msg(&mut self, stream: &mut UnixStream) -> io::Result<String> {
		// Get message length
		let msg_len = stream.read_u16().await?;

		// Read message
		let msg_buf = &mut self.buf[0..msg_len as usize];
		stream.read(msg_buf).await?;
		let msg = String::from_utf8_lossy(msg_buf).to_string();

		Ok(msg)
	}
}

// Write an IPC message
pub async fn write_msg(stream: &mut UnixStream, msg: &str) -> io::Result<()> {
	// Encode and write message size header 
	if msg.len() > BUF_SIZE {
		let error = io::Error::new(
			io::ErrorKind::InvalidData,
			format!("message length exceeds buffer size of {}", BUF_SIZE));
		return Err(error);
	}
	stream.write_u16(msg.len() as u16).await?;

	// Write message
	stream.write(msg.as_bytes()).await?;

	Ok(())
}
