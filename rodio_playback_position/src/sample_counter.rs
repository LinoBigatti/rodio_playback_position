use cpal::StreamInstant;

type SampleType = u64;

pub struct SampleTimestamp {
    instant: StreamInstant,
    sample_n: SampleType,
}

impl SampleTimestamp {
    pub fn new(instant: StreamInstant, sample_n: SampleType) -> Self {
        Self {
            instant,
            sample_n,
        }
    }
}

pub type SampleCounter Arc<SampleCounterImpl>;

pub struct SampleCounterImpl {
    samples_per_second: u32,
    nanos_per_sample: f32,
    
    stream_start: Option<StreamInstant>;
    
    /// The last confirmed sample reading: This should be in the past already, with a confirmed
    /// sample count.
    last_sample_timestamp: SampleTimestamp,

    /// The next predicted sample timestamp: The timestamp at which cpal thinks it will deliver the
    /// next batch of samples. This should be updated as soon as they are ready.
    next_sample_timestamp: SampleTimestamp,
} 
