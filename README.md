# rodio_playback_position: Track the Playback Position of a Rodio Source at runtime.

[![Crates.io](https://img.shields.io/crates/v/rodio_playback_position.svg)](https://crates.io/crates/rodio_playback_position)
[![Docs.rs](https://docs.rs/rodio_playback_position/badge.svg)](https://docs.rs/rodio_playback_position)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A crate that provides a small playback backend for [rodio](https://github.com/RustAudio/rodio) sources, 
with support for a high-precision interpolated playback position counter.

This is useful for applications that need to sync visuals or other events with audio playback at runtime, 
such as rhythm games or music players. For scheduling and synchronizing the playback of multiple sources
simultaneously, see the [rodio_scheduler](https://crates.io/crates/rodio_scheduler) crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rodio_playback_position = { version = "0.1.3" }
# Or alternatively, for 128 bit sample counters:
#rodio_playback_position = { version = "0.1.3", features = ["u128"] }
rodio = "0.21.1" 
cpal = "0.16.0"
```

The `u128` feature flag can be enabled to use `u128` as the `SampleType`, increasing 
the maximum uptime of the library, at the cost of increased memory usage.

Here is an example of how to use the library to start playback and keep track of the current playback position. 
For more information, see the [Docs](https://docs.rs/rodio_scheduler), as well as the [cpal Docs](https://docs.rs/cpal).

```rust
use std::time::Duration;
use cpal::traits::{HostTrait, DeviceTrait};
use rodio::source::SineWave;
use rodio::Source;
use rodio_playback_position::{OutputStreamConfig, stream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get a cpal output device.
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    println!("Using output device: {}", device.name()?);

    // Create a config for the stream.
    let config = OutputStreamConfig::from(device.default_output_config().unwrap());

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
}
