use std::{error::Error, io};
use super::message::{Response, STATUS_FAIL, STATUS_SUCCESS};
use tokio::{net::UnixStream, io::AsyncWriteExt};
use lazy_static::lazy_static;

// RESP_PREPARE_ERR is prepared statically to prevent recursion
lazy_static! {
	static ref RESP_PREPARE_ERR: Response = Response::from(STATUS_FAIL, "Failed to prepare error response.".to_string()).unwrap();
}
/// Handle command message and write response
pub async fn handle_command(cmd: &str, mut stream: UnixStream) {
	let resp = match cmd {
		"status" => {
			Response::from(STATUS_SUCCESS, "Status message".to_string())
		},
		msg => Response::from(STATUS_FAIL, format!("Unknown command: {}", msg))
	};

	let send_result = match resp {
		Ok(resp) => resp.encode(&mut stream).await, // Send a success response
		Err(e) => send_error(Box::new(e), &mut stream).await
	};
	if let Err(e) = send_result {
		eprintln!("Failed to send message: {}", e);
	}

	// Close the stream after sending a response
	if let Err(e) = stream.shutdown().await {
		eprintln!("Failed to close stream: {}", e);
		// If an error was encountered, the stream will still be closed when dropped, but
		// not gracefully
	}
}

// Log and attempt to send IPC error messages
async fn send_error(err: Box<dyn Error + Send>, stream: &mut UnixStream) -> Result<(), io::Error> {
	let err_msg = format!("Error: {}", err);
	eprintln!("{}", &err_msg);

	match Response::from(STATUS_FAIL, err_msg) {
		Ok(err_resp) => err_resp.encode(stream).await,
		Err(e) => {
			eprintln!("Failed to prepare error response: {}", e);
			RESP_PREPARE_ERR.encode(stream).await
		}
	}
}
