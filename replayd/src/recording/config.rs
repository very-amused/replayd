use serde::Deserialize;
use super::error::RecordingError;

/// Recording config
#[derive(Deserialize)]
pub struct Config {
	#[serde(default = "Config::default_video_config")]
	video: VideoConfig,
	#[serde(default = "Config::default_audio_config")]
	audio: AudioConfig 
}

impl Config {
	fn default_video_config() -> VideoConfig {
		VideoConfig {
			encoder: "h264_nvenc".to_string()
		}
	}
	fn default_audio_config() -> AudioConfig {
		AudioConfig {
			encoder: "aac".to_string(),
			samplerate: 48000,
			sampledepth: 24
		}
	}
}

#[derive(Deserialize)]
pub struct VideoConfig {
	/// Encoder used in both recording and saving clips,
	/// hardware acceleration highly recommended
	pub encoder: String
}

#[derive(Deserialize)]
pub struct AudioConfig {
	/// Encoder used when saving clips
	/// (audio is recorded live as lossless pcm to avoid CPU overhead)
	pub encoder: String,

	/// Sample rate used for recording
	pub samplerate: u32,
	/// Sample depth used for recording
	pub sampledepth: u8
}
