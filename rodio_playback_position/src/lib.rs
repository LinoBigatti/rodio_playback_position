/*!
A minimal playback library providing high-precision tracking of the playback position.

`rodio_scheduler` provides a simple playback backend for [rodio](https://crates.io/crates/rodio) `Source`s which can keep track
of the playback position of the stream. It is useful for applications that need to synchronize
visuals or other events with audio playback at runtime, such as rhythm games or music players.
For scheduling and synchronizing the playback of multiple sources before starting playback, see
the [rodio_scheduler](https://crates.io/crates/rodio_scheduler) crate.

The core of the crate is the [`stream::open`] function, which creates an audio output stream
and returns a [`StreamHandle`]. The [`StreamHandle`] can then be used to get the current
playback position in samples using the [`StreamHandle::sample_count`] method.

The reported sample count is interpolated between audio buffer updates, providing a
smoother and higher-precision estimate of the playback position than what is typically
available from audio APIs.

## Example

The following example shows how to use the library to start playback of a sine wave and keep track of the current playback position.
For more information on configuring the audio, see the [cpal Docs](https://docs.rs/cpal). For more information in `Source`s, see the
[rodio Docs](https://docs.rs/rodio).

```no_run
use std::time::Duration;
use rodio::source::SineWave;
use rodio::Source;
use rodio_playback_position::{OutputStreamConfig, stream};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a cpal output device.
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    println!("Using output device: {}", device.name()?);

    // Create a config for the stream.
    let config = OutputStreamConfig::from(&device.default_output_config().unwrap());

    // Create a source.
    let source = SineWave::new(440.0).take_duration(Duration::from_secs(5));

    // Create a stream handle.
    println!("Opening stream...");
    let mut stream_handle = stream::open(
        &device,
        &config,
        source,
        |err| eprintln!("stream error: {}", err),
    )?;
    println!("Stream opened.");

    // Get the sample count.
    for _ in 0..50 {
        println!("Sample count: {}", stream_handle.sample_count());
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
# }
```
*/

/// The type used to count samples across the crate.
type SampleType = u64;

mod config;
pub use config::OutputStreamConfig;

mod error;
pub use error::StreamError;

mod sample;
use sample::SampleTimestamp;

mod sample_timestamp_buffer;
use sample_timestamp_buffer::{BufferConsumer, BufferProducer, new_sample_timestamp_buffer};

pub mod stream;
pub use stream::StreamHandle;
