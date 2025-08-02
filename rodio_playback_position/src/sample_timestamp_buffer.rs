use std::sync::Arc;

use ringbuf::{HeapRb, CachingProd, CachingCons, traits::*};

use crate::SampleTimestamp;

/// A lock-free ring buffer producer for `SampleTimestamp`s, designed for a single producer thread.
pub struct BufferProducer {
    producer: CachingProd<Arc<HeapRb<SampleTimestamp>>>,
}

impl BufferProducer {
    /// Adds a new `SampleTimestamp` to the buffer.
    /// This is a lock-free operation.
    #[inline]
    pub fn add(&mut self, timestamp: SampleTimestamp) {
        self.producer.try_push(timestamp).ok(); // Ignore errors on full buffer
    }
}

/// A lock-free ring buffer consumer for `SampleTimestamp`s, designed to be queried by non-critical threads.
pub struct BufferConsumer {
    consumer: CachingCons<Arc<HeapRb<SampleTimestamp>>>,
}

impl BufferConsumer {
    /// Removes the eldest item from the ring buffer and returns it.
    ///
    /// Returns `None` if the ring buffer is empty.
    pub fn pop_oldest(&mut self) -> Option<SampleTimestamp> {
        self.consumer.try_pop()
    }
    
    /// Returns a reference to the most recent item in the ring buffer, if exists.
    ///
    /// *Returned item may not be actually the most recent if there is a concurrent producer activity.*
    pub fn newest(&self) -> Option<&SampleTimestamp> {
        self.consumer.last()
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
) -> (
    BufferProducer,
    BufferConsumer,
) {
    let buffer = HeapRb::new(capacity);
    let (producer, consumer) = buffer.split();

    (BufferProducer { producer }, BufferConsumer { consumer })
}
