use std::sync::Arc;

use cpal::StreamInstant;

use crate::{SampleType, BufferConsumer};

pub struct SampleCounter {
    samples_per_second: u32,
    nanos_per_sample: f32,

    stream_start: Option<StreamInstant>,

    sample_timestamp_buffer_consumer: BufferConsumer,
}

impl SampleCounter {
    pub fn new(sample_rate: u32, sample_timestamp_buffer_consumer: BufferConsumer) -> Arc<Self> {
        let nanos_per_sample = 1_000_000_000.0 / sample_rate as f32;

        Arc::new(Self {
            samples_per_second: sample_rate,
            nanos_per_sample,
            stream_start: None,
            sample_timestamp_buffer_consumer,
        })
    } 

    pub fn get_samples(&self) -> SampleType {
        match self.sample_timestamp_buffer_consumer.newest() {
            Some(t) => t.sample_n,
            None => 0,
        }
    }
}
