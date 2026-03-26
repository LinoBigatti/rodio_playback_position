use rtsan_standalone::nonblocking;

use crate::sample::SampleTimestamp;

/// A lock-free buffer producer for `SampleTimestamp`s, designed for a single producer thread.
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

/// A lock-free buffer consumer for `SampleTimestamp`s, designed to be queried by non-critical threads.
pub struct BufferConsumer {
    consumer: triple_buffer::Output<SampleTimestamp>,
}

impl BufferConsumer {
    /// Returns a reference to the most recent item in the buffer.
    pub fn newest(&mut self) -> &SampleTimestamp {
        self.consumer.read()
    }
}

/// Creates a new sample timestamp triple buffer.
///
/// # Returns
///
/// A tuple containing the producer and consumer ends of the buffer.
pub fn new_sample_timestamp_buffer() -> (BufferProducer, BufferConsumer) {
    let default_element = SampleTimestamp::default();
    let (producer, consumer) = triple_buffer::triple_buffer(&default_element);

    (BufferProducer { producer }, BufferConsumer { consumer })
}
