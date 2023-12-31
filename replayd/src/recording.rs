use chrono::{self, TimeZone};
use std::time::Duration;
use tokio::{time::{self, Interval}, signal::unix::Signal};
use error::RecordingError;
use config::Config;
use ffmpeg_next::{self as ffmpeg, codec::codec::Codec};

pub mod config;
pub mod error;

/// A desktop recorder which uses 1s chunks
/// Must run on a separate thread, channels are
/// used to update state and copy frame references
///
/// # Safety
/// libavcodec uses reference counting for packet structs since version 4.0.0,
/// TODO
pub struct Recorder {
	interval: Interval,
	
	pub config: Option<Config>
}

impl Recorder {
	/// Create a new Recorder with a running interval
	pub async fn new(sigint: &mut Signal, sigterm: &mut Signal) -> Option<Self> {
		// Wait until next even second to start interval
		let interval = {
			let mut to_second = time::interval(to_next_second());
			to_second.tick().await;
			tokio::select! {
				_ = to_second.tick() => {
					// Start second interval ticker
					let mut interval = time::interval(Duration::from_secs(1));
					// The interval should stay ticking regardless of whether recording is enabled/disabled,
					// so missed ticks are expected
					interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
					interval
				},
				_ = sigint.recv() => return None,
				_ = sigterm.recv() => return None
			}
		};

		Some(Self{interval, config: None})
	}

	pub async fn start(&mut self) {
	}
}


/// Duration until next second
/// copied from my_timers
fn to_next_second() -> Duration {
	let now = chrono::Utc::now();
	let next = chrono::Utc.timestamp_opt(
		now.timestamp()+1, 0).unwrap();

	(next - now).to_std().unwrap()
}
