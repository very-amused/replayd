use std::{fs, io, fmt::{Display, Formatter}};


/// Get the path for replayd's pidfile
pub fn get_pidfile(runtime_dir: &str) -> String {
	const PIDFILE: &str = "replayd.pid";
	format!("{}/{}", runtime_dir, PIDFILE)
}

/// Read the replayd's pidfile
pub fn read_pidfile(pidfile: &str) -> Option<u32> {
	match fs::read_to_string(pidfile) {
		Ok(file) => file.parse::<u32>().ok(),
		_ => None
	}
}

/// Set the PID in replayd's pidfile
pub fn write_pidfile(pidfile: &str, pid: u32) -> io::Result<()> {
	// Open file for writing
	fs::write(pidfile, pid.to_string())
}

#[derive(Debug)]
pub enum Error {
	// pid, pidfile
	InstanceRunning(u32)
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InstanceRunning(pid) => write!(f, "replayd is already running (pid {})", 
				pid)
		}
	}

}

impl std::error::Error for Error {}
