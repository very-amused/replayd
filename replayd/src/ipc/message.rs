#![allow(dead_code)]
use std::{error::Error, fmt::Display, io::{self, Write, Read}};
use std::os::unix::net::UnixStream as StdUnixStream;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

/// The command has run successfully, exit 0
pub const STATUS_SUCCESS: u8 = b'\x06';
/// The command has failed, exit 1
pub const STATUS_FAIL: u8 = b'\x07';

const STATUS_EMPTY: u8 = b'\x00';

/// Message decoding buffer size
pub const BUF_SIZE: usize = 8192;

/// Check if a message body is within BUF_SIZE
fn check_body_size(body: &str) -> io::Result<()> {
		if body.len() > BUF_SIZE {
			let err = io::Error::new(
				io::ErrorKind::InvalidData,
				format!("message body length exceeds BUF_SIZE ({})", BUF_SIZE));
			return Err(err)
		}
		Ok(())
}

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
		check_body_size(&body)?;
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
	body: String
}

impl Response {
	pub fn body(&self) -> &str {
		&self.body
	}
	// Return Result used to determine exit code
	pub fn check(&self) -> Result<(), StatusError> {
		match self.status {
			STATUS_SUCCESS => Ok(()),
			STATUS_FAIL => Err(StatusError::StatusFail(self.body.clone())),
			s => Err(StatusError::StatusUnknown(s))
		}
	}
	pub fn from(status: u8, body: String) -> io::Result<Self> {
		check_body_size(&body)?;
		Ok(Self{size: body.len() as u16, status, body})
	}

	pub fn decode(stream: &mut StdUnixStream, buf: &mut [u8; BUF_SIZE]) -> io::Result<Self> {
		// Read header values
		let (size, status) = {
			let b = &mut buf[0..3];
			stream.read_exact(b)?;
			(u16::from_be_bytes([b[0], b[1]]), b[2])
		};
		// Read body
		let body = {
			let b = &mut buf[0..size as usize];
			stream.read(b)?;
			// Decode and copy to string
			String::from_utf8_lossy(b).to_string()
		};
		
		Ok(Self{size, status, body})
	}

	pub async fn encode(&self, stream: &mut UnixStream) -> io::Result<()> {
		// size is known to be accurate
		let size = self.size.to_be_bytes();
		stream.write_all(&size).await?;
		// Write status and message
		stream.write_all(&[self.status]).await?;
		stream.write_all(self.body.as_bytes()).await?;

		Ok(())
	}
}

#[derive(Debug)]
pub enum StatusError {
	StatusFail(String),
	StatusUnknown(u8)
}

impl Display for StatusError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::StatusFail(body) => write!(f, "{}", body),
			Self::StatusUnknown(s) => write!(f, "Received unknown response status: {}", s)
		}
	}
}

impl Error for StatusError {}
