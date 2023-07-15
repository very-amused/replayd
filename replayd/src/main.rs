use std::{error::Error, io};

use tokio::{net::UnixListener, signal::unix::{signal, SignalKind}, task::{JoinHandle, JoinSet}};

mod ipc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	// Create socket
	let socket_path = ipc::socket::socket_path();
	println!("Listening on {}", socket_path);
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
	

	// Wait for connections until SIGINT is sent
	loop {
		tokio::select! {
			stream = sock.accept() => match stream {
				Ok(stream) => {
					todo!();
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
	if let Some(thread) = record_thread {
		println!("Stopping recording");
		todo!();
	}
	if let Some(threads) = save_threads {
		println!("Saving pending clips");
		todo!();
	}

	Ok(())
}
