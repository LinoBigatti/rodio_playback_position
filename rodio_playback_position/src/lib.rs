/// The type used to count samples across the crate.
type SampleType = u64; 

mod config;
pub use config::OutputStreamConfig;

mod error;
pub use error::StreamError;

mod sample;
use sample::SampleTimestamp;

mod sample_timestamp_buffer;
use sample_timestamp_buffer::{BufferProducer, BufferConsumer, new_sample_timestamp_buffer};

pub mod stream;
pub use stream::StreamHandle;
