use std::{io::{self, Write, Read}, u16};
use super::message::BUF_SIZE;
use tokio::{net::UnixStream, io::{AsyncReadExt,AsyncWriteExt}};
use std::os::unix::net::UnixStream as StdUnixStream;

pub fn get_socket_path(runtime_dir: &str) -> String {
	const SOCKET_NAME: &str = "replayd.sock";
	format!("{}/{}", runtime_dir, SOCKET_NAME)
}
	
/// A reusable read buffer for IPC messages
pub struct ReadBuffer {
	buf: [u8; BUF_SIZE]
}

impl ReadBuffer {
	// Allocate a new ReadBuffer
	pub fn new() -> Self {
		ReadBuffer { buf: [0; BUF_SIZE] }
	}

	// Read an IPC response
	#[allow(dead_code)]
	pub fn read_resp(&mut self, stream: &mut StdUnixStream) -> io::Result<(u8, String)> {
		// Get message length from size header
		let mut msg_len = [0 as u8; 2];
		stream.read_exact(&mut msg_len)?;
		let msg_len = u16::from_be_bytes(msg_len);

		// Read status
		let mut status = [0 as u8];
		stream.read_exact(&mut status)?;

		// Read message
		let msg_buf = &mut self.buf[0..msg_len as usize];
		stream.read(msg_buf)?;
		let msg = String::from_utf8_lossy(msg_buf).to_string();

		Ok((status[0], msg))
	}
}

// Write an IPC response 
pub async fn write_resp(stream: &mut UnixStream, resp: (u8, &str)) -> io::Result<()> {
	// Encode and write message size header 
	if resp.1.len() > BUF_SIZE {
		let error = io::Error::new(
			io::ErrorKind::InvalidData,
			format!("message length exceeds buffer size of {}", BUF_SIZE));
		return Err(error);
	}
	stream.write_u16(resp.1.len() as u16).await?;

	// Write status
	stream.write_u8(resp.0).await?;

	// Write message
	stream.write(resp.1.as_bytes()).await?;

	Ok(())
}

