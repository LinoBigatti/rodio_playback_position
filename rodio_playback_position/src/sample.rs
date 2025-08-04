use std::time::Duration;

use crate::SampleType;

#[derive(Clone, Copy, Debug)]
pub struct SampleTimestamp {
    /// A duration from an unspecified start time ocurring at or before the stream was opened.
    timestamp: Duration,
    sample_n: SampleType,
}

impl SampleTimestamp {
    #[inline]
    pub fn new(os_timestamp: Instant, start_time: Instant, sample_n: SampleType) -> Self {
        let duration = os_timestamp.checked_duration_since(start_time).unwrap_or_default();
        
        Self {
            timestamp: duration,
            sample_n,
        }
    }

    pub fn interpolate(other)
}

