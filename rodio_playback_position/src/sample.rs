use std::time::Duration;

use crate::SampleType;

const NANOS_PER_SEC: SampleType = 1_000_000_000;

#[derive(Default, Clone, Copy, Debug)]
/// Timestamp data for an audio playback event. This type is created when the audio thread produces
/// a batch of samples.
///
/// You should not create this type directly. Instead, use [StreamHandle::sample_count](crate::StreamHandle::sample_count)
/// to get an interpolated sample position.
pub struct SampleTimestamp {
    /// This timestamp represents a duration since some unspecified start time occurring either
    /// before or equal to the moment the stream from which it was created begins.
    timestamp: Duration,

    /// The sample number being sent to the playback thread at this point in time.
    sample_n: SampleType,

    /// The predicted latency of the audio stream. This is the difference between when the samples
    /// are sent to be processend and the moment they will be played back by the output device.
    latency: Duration,
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
    pub fn interpolate(&self, current_timestamp: Duration, sample_rate: u32) -> SampleType {
        let _sample_rate = sample_rate as SampleType;

        let mut sample_n: SampleType = self.sample_n;

        // Potentially truncating if SampleType is not u128. Unlikely to appear in real life
        // scenarios, as the time difference should usually be less than a second.
        let time_diff_nanos = self.timestamp.abs_diff(current_timestamp).as_nanos() as SampleType;
        let time_diff_samples = _sample_rate * time_diff_nanos / NANOS_PER_SEC;

        if current_timestamp < self.timestamp {
            sample_n -= time_diff_samples;
        } else {
            sample_n += time_diff_samples;
        }

        // Same truncating considerations as above. Even more unlikely to happen in real life.
        let latency_samples = _sample_rate * self.latency.as_nanos() as SampleType / NANOS_PER_SEC;

        // Apply audio latency.
        if sample_n > latency_samples {
            sample_n -= latency_samples;
        } else {
            // The stream data has not started playing in the output device yet.
            return 0;
        }

        sample_n
    }
}
