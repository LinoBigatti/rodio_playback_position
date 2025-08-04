use cpal::StreamInstant;

use crate::{SampleType, SampleTimestamp, BufferConsumer};

pub struct SampleCounter {
    samples_per_second: u32,
    nanos_per_sample: f32,

    stream_start: Option<SampleTimestamp>,

    sample_timestamp_buffer_consumer: BufferConsumer,
}

impl SampleCounter {
    pub fn new(sample_rate: u32, sample_timestamp_buffer_consumer: BufferConsumer) -> Self {
        let nanos_per_sample = 1_000_000_000.0 / sample_rate as f32;

        Self {
            samples_per_second: sample_rate,
            nanos_per_sample,
            stream_start: None,
            sample_timestamp_buffer_consumer,
        }
    } 

    pub fn get_samples(&mut self) -> SampleType {
        let latest_sample_timestamp = self.sample_timestamp_buffer_consumer.newest();
        
        latest_sample_timestamp.sample_n

        //if self.stream_start.is_none() {
            //self.stream_start = Some(latest_sample_timestamp)
        //}
    }
}
