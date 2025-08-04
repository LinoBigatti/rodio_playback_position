use rtsan_standalone::nonblocking;

use crate::SampleTimestamp;

/// A lock-free ring buffer producer for `SampleTimestamp`s, designed for a single producer thread.
pub struct BufferProducer {
    producer: triple_buffer::Input<SampleTimestamp>,
}

impl BufferProducer {
    /// Adds a new `SampleTimestamp` to the buffer.
    /// This is a realtime-safe operation.
    #[inline]
    #[nonblocking]
    pub fn update(&mut self, timestamp: SampleTimestamp) {
        self.producer.write(timestamp)
    }
}

/// A lock-free ring buffer consumer for `SampleTimestamp`s, designed to be queried by non-critical threads.
pub struct BufferConsumer {
    consumer: triple_buffer::Output<SampleTimestamp>,
}

impl BufferConsumer {
    /// Returns a reference to the most recent item in the buffer.
    pub fn newest(&mut self) -> &SampleTimestamp {
        self.consumer.read()
    }
}

/// Creates a new sample timestamp ring buffer.
///
/// # Arguments
///
/// * `capacity` - The capacity of the ring buffer.
///
/// # Returns
///
/// A tuple containing the producer and consumer ends of the ring buffer.
pub fn new_sample_timestamp_buffer(
    capacity: usize,
    start_time: std::time::Instant,
) -> (
    BufferProducer,
    BufferConsumer,
) {
    let default_element = SampleTimestamp::new(
        start_time,
        0,
    );
    let (producer, consumer) = triple_buffer::triple_buffer(&default_element);

    (BufferProducer { producer }, BufferConsumer { consumer })
}
