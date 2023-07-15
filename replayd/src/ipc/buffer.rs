use std::{io, u16};

use tokio::{net::UnixStream, io::{AsyncReadExt,AsyncWriteExt}};

// Size of reusable message buffer
pub const BUF_SIZE: usize = 8192;
// Message headers are a single u16 describing the message's size in the range 0-BUFSIZ
const HEADER_SIZE: usize = 2;

/// A reusable read buffer for IPC messages
pub struct Buffer {
	buf: [u8; BUF_SIZE]
}

impl Buffer {
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
