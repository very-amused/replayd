use tokio::net::UnixListener;
use tokio::{net::UnixStream, io::Interest};
use tokio::io;
use std::env;
use users::get_current_uid;

pub const SOCKET_NAME: &str = "replayd.sock";

/// Get the path of the Unix socket used for IPC
pub fn socket_path() -> String {
	// Try to use XDG_RUNTIME_DIR, otherwise default to /run/user/$uid
	let runtime_dir: String = {
		if let Ok(path) = env::var("XDG_RUNTIME_DIR") {
			path
		} else {
			format!("/run/user/{}", get_current_uid())
		}
	};
	const SOCKET_NAME: &str = "replayd.sock";

	format!("{}/{}", runtime_dir, SOCKET_NAME)
}
	
