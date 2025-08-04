use rodio::{ChannelCount, SampleRate};

use cpal::{BufferSize, SampleFormat, StreamConfig, SupportedStreamConfig};

const HZ_48000: u32 = 48_000;

/// Describes the output stream's configuration. We have to reimplement this here because rodio
/// doesn't include this struct when the `playback` feature is disabled.
#[derive(Copy, Clone, Debug)]
pub struct OutputStreamConfig {
    pub channel_count: ChannelCount,
    pub sample_rate: SampleRate,
    pub buffer_size: BufferSize,
    pub sample_format: SampleFormat,
}

impl Default for OutputStreamConfig {
    fn default() -> Self {
        Self {
            channel_count: 2,
            sample_rate: HZ_48000,
            buffer_size: BufferSize::Default,
            sample_format: SampleFormat::F32,
        }
    }
}

impl From<&OutputStreamConfig> for StreamConfig {
    fn from(config: &OutputStreamConfig) -> Self {
        cpal::StreamConfig {
            channels: config.channel_count as cpal::ChannelCount,
            sample_rate: cpal::SampleRate(config.sample_rate),
            buffer_size: config.buffer_size,
        }
    }
}

impl From<StreamConfig> for OutputStreamConfig {
    fn from(config: StreamConfig) -> Self {
        OutputStreamConfig {
            channel_count: config.channels as ChannelCount,
            sample_rate: config.sample_rate.0,
            buffer_size: config.buffer_size,
            sample_format: SampleFormat::F32,
        }
    }
}

impl From<SupportedStreamConfig> for OutputStreamConfig {
    fn from(config: SupportedStreamConfig) -> Self {
        OutputStreamConfig::from(
            Into::<StreamConfig>::into(config)
        )
    }
}

impl OutputStreamConfig {
    /// Access the output stream config's channel count.
    pub fn channel_count(&self) -> ChannelCount {
        self.channel_count
    }

    /// Access the output stream config's sample rate.
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    /// Access the output stream config's buffer size.
    pub fn buffer_size(&self) -> &BufferSize {
        &self.buffer_size
    }

    /// Access the output stream config's sample format.
    pub fn sample_format(&self) -> SampleFormat {
        self.sample_format
    }
}
