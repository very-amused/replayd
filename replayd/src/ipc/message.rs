#![allow(dead_code)]
use std::{error::Error, fmt::Display};
use std::os::unix::net::UnixStream as StdUnixStream;

/// The command has run successfully, exit 0
pub const STATUS_SUCCESS: u8 = b'\x06';
/// The command has failed, exit 1
pub const STATUS_FAIL: u8 = b'\x07';


/// An IPC message sent client -> server
pub struct Message {
	/// Body size in bytes
	size: u16,
	/// Body text
	pub body: String
}

impl Message {
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
