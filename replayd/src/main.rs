use std::{error::Error, io, process, fs};
use tokio::{net::UnixListener, signal::unix::{signal, SignalKind}, task::{JoinHandle, JoinSet}};
use sysinfo::{System,SystemExt, ProcessExt, PidExt};

mod ipc;
mod pid;
mod util;
use ipc::socket;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let runtime_dir = util::runtime_dir();

	// Check if a replayd process is already running
	let pidfile = pid::get_pidfile(&runtime_dir);
	{
		let mut sys = System::new();
		if let Some(old_pid) = pid::read_pidfile(&pidfile) {
			// Error if pid is still running, otherwise ignore (set_pid will truncate)
			if sys.refresh_process((old_pid as usize).into()) {
				let err = pid::Error::InstanceRunning(old_pid);
				eprintln!("{}", err);
				return Err(err.into());
			}
		} else {
			// Check if any replayd processes are running
			sys.refresh_processes();
			if let Some(process) = sys.processes_by_name("replayd").next() {
				let err = pid::Error::InstanceRunning(process.pid().as_u32());
				eprintln!("{}", err);
				return Err(err.into());
			}
		}
	}
	// Create pidfile
	let active_pid = process::id();
	pid::write_pidfile(&pidfile, active_pid).map_err(|err| {
		eprintln!("Failed to set PID");
		err
	})?;

	// Create socket
	let socket_path = socket::get_socket_path(&runtime_dir);
	fs::remove_file(&socket_path).map_err(|err| {
		eprintln!("Failed to remove old socket");
		err
	})?;
	println!("Listening on {}", &socket_path);
	let sock = UnixListener::bind(&socket_path)?;

	// Listen for SIGINT/SIGTERM to safely shutdown.
	// tokio::select! must be used to catch these signals
	// for all future awaits on the main thread
	let sigint = signal(SignalKind::interrupt())?;
	let sigterm = signal(SignalKind::terminate())?;
	tokio::pin!(sigint);
	tokio::pin!(sigterm);

	// Start recording thread
	// TODO: configurable recording on start
	
	// Allocate IPC message read buffer
	let mut ipc_readbuf = socket::ReadBuffer::new();
	

	// Wait for connections until SIGINT is sent
	loop {
		tokio::select! {
			stream = sock.accept() => match stream {
				Ok((mut stream, _)) => {
					// Read message
					if let Ok(msg) = ipc_readbuf.read_msg(&mut stream).await {
						println!("{}", msg);
					} else {
						// Notify the client that the message could not be read
						socket::write_msg(&mut stream, "Error").await.unwrap_or_else(|err| {
							eprintln!("Failed to write error: {}", err);
						});
						continue;
					}
				}
				Err(err) => {
					eprintln!("Failed to accept connection: {}", err);
					continue;
				}
			},
			_ = sigint.recv() => return shutdown(None, None).await,
			_ = sigterm.recv() => return shutdown(None, None).await
		}
	}
}

async fn shutdown(
	record_thread: Option<JoinHandle<()>>,
	save_threads: Option<JoinSet<io::Result<()>>>) -> Result<(), Box<dyn Error>> {
	println!("Shutting down");
	if let Some(_thread) = record_thread {
		println!("Stopping recording");
		todo!();
	}
	if let Some(_threads) = save_threads {
		println!("Saving pending clips");
		todo!();
	}

	Ok(())
}
