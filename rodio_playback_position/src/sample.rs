use std::time::{Instant, Duration};

use crate::SampleType;

#[derive(Default, Clone, Copy, Debug)]
pub struct SampleTimestamp {
    /// This timestamp represents a duration since some unspecified start time occurring either 
    /// before or equal to the moment the stream from which it was created begins.
    timestamp: Duration,

    /// The sample number being sent to the playback thread at this point in time.
    sample_n: SampleType,

    /// The predicted latency of the audio stream. This is the difference between when the samples
    /// are sent to be processend and the moment they will be played back by the output device.
    latency: Duration
}

impl SampleTimestamp {
    #[inline]
    pub fn new(timestamp: Duration, latency: Duration, sample_n: SampleType) -> Self {
        Self {
            timestamp,
            sample_n,
            latency,
        }
    }

    #[inline]
    pub fn interpolate(&self, current_timestamp: Duration, sample_rate: u64) -> SampleType {
        let mut sample_n: SampleType = self.sample_n;

        let time_diff = self.timestamp.abs_diff(current_timestamp);

        let nanos_per_sample = 1_000_000_000 / sample_rate;

        let time_diff_samples = time_diff.as_secs() * sample_rate + time_diff.subsec_nanos() as u64 / nanos_per_sample;

        if current_timestamp < self.timestamp {
            sample_n -= time_diff_samples;
        } else {
            sample_n += time_diff_samples;
        }

        let latency_samples = self.latency.as_secs() * sample_rate + self.latency.subsec_nanos() as u64 / nanos_per_sample;
        sample_n += latency_samples;

        sample_n
    }
}

