use chrono::{self, TimeZone};
use std::time::Duration;
use tokio::{time::{self, Interval}, signal::unix::Signal};

/// A desktop recorder which uses 1s chunks
/// Must run on a separate thread, channels are
/// used to update state and copy frame references
///
/// # Safety
/// libavcodec uses reference counting for packet structs since version 4.0.0,
/// TODO
pub struct Recorder {
	interval: Interval
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

		Some(Self{interval})
	}
}

// Testing constants
// TODO: remove when config is implemented


/// Duration until next second
/// copied from my_timers
fn to_next_second() -> Duration {
	let next = chrono::Utc.timestamp_opt(
		chrono::Utc::now().timestamp()+1, 0).unwrap();

	let now = chrono::Utc::now(); // Get duration against current time
	// Round up to next ms
	(next - now).to_std().unwrap()
}
