use cpal::StreamInstant;

use crate::SampleType;

#[derive(Clone, Copy, Debug)]
pub struct SampleTimestamp {
    pub instant: StreamInstant,
    pub sample_n: SampleType,
}

impl SampleTimestamp {
    #[inline]
    pub fn new(instant: StreamInstant, sample_n: SampleType) -> Self {
        Self {
            instant,
            sample_n,
        }
    }
}

