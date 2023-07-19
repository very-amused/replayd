use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum RecordingError {
	CodecNotFound(String)
}

impl Display for RecordingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::CodecNotFound(codec) => write!(f, "Codec not found: {}", codec)
		}
	}

}

impl Error for RecordingError {}
