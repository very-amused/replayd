#![allow(dead_code)]
use std::{error::Error, fmt::Display, io::{self, Write, Read}};
use std::os::unix::net::UnixStream as StdUnixStream;

use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;

/// The command has run successfully, exit 0
pub const STATUS_SUCCESS: u8 = b'\x06';
/// The command has failed, exit 1
pub const STATUS_FAIL: u8 = b'\x07';

/// Message decoding buffer size
pub const BUF_SIZE: usize = 8192;

/// An IPC message sent client -> server
pub struct Message {
	/// Body size in bytes
	size: u16,
	/// Body text
	body: String
}

impl Message {
	pub fn body(&self) -> &str {
		return &self.body
	}
	pub fn from(body: String) -> io::Result<Self> {
		if body.len() > BUF_SIZE {
			let err = io::Error::new(
				io::ErrorKind::InvalidData,
				format!("message body length exceeds BUF_SIZE ({})", BUF_SIZE));
			return Err(err);
		}
		Ok(Self {size: body.len() as u16, body})
	}


	pub async fn decode(stream: &mut UnixStream, buf: &mut [u8; BUF_SIZE]) -> io::Result<Self> {
		// Read header
		let size = {
			let b = &mut buf[0..2];
			stream.read_exact(b).await?;
			u16::from_be_bytes([b[0], b[1]])
		};
		// Read body
		let body = {
			let b = &mut buf[0..size as usize];
			stream.read(b).await?;
			// Decode and copy to String
			String::from_utf8_lossy(b).to_string()
		};

		Ok(Self{size, body})
	}
	pub fn encode(&self, stream: &mut StdUnixStream) -> io::Result<()> {
		// size is known to be accurate
		let size = self.size.to_be_bytes();
		stream.write_all(&size)?;
		// Write message
		stream.write_all(self.body.as_bytes())?;
		Ok(())
	}
}

/// An IPC response sent server -> client
pub struct Response {
	/// Body size in bytes
	size: u16,
	/// Whether the IPC command was successful,
	/// STATUS_SUCCESS | STATUS_FAIL
	status: u8,
	/// Body text
	pub body: String
}

impl Response {
	// Return Result used to determine exit code
	pub fn check(&self) -> Result<(), StatusError> {
		match self.status {
			STATUS_SUCCESS => Ok(()),
			STATUS_FAIL => Err(StatusError::StatusFail(&self.body)),
			s => Err(StatusError::StatusUnknown(s))
		}
	}
}

#[derive(Debug)]
pub enum StatusError<'a> {
	StatusFail(&'a str),
	StatusUnknown(u8)
}

impl<'a> Display for StatusError<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::StatusFail(body) => write!(f, "{}", body),
			Self::StatusUnknown(s) => write!(f, "Received unknown response status: {}", s)
		}
	}
}

impl<'a> Error for StatusError<'a> {}
