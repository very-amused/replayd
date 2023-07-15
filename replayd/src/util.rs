use std::env;
use users::get_current_uid;

/// Get the path of the user's runtime directory
pub fn runtime_dir() -> String {
	// Try to use XDG_RUNTIME_DIR, otherwise default to /run/user/$uid
	if let Ok(path) = env::var("XDG_RUNTIME_DIR") {
		path
	} else {
		format!("/run/user/{}", get_current_uid())
	}
}

