use std::sync::{Arc, atomic::AtomicU64};

use rodio::Source;

use cpal::Sample;
use cpal::traits::{DeviceTrait, StreamTrait};

use crate::{StreamError, OutputStreamConfig, SampleType, SampleCounter, SampleTimestamp, BufferProducer, BufferConsumer};

/// A handle for accessing and communicating with the audio stream.
///
/// If this struct is dropped, playback will stop.
pub struct StreamHandle {
    _handle: cpal::Stream,
    sample_counter: Arc<AtomicU64>,
    sample_timestamp_consumer: BufferConsumer,
}

impl StreamHandle {
    /// Returns a reference to the sample counter tracking this stream.
    pub fn sample_counter(&mut self) -> Arc<AtomicU64> {
        let timestamp = self.sample_timestamp_consumer.newest();
        self.sample_counter.store(timestamp.sample_n, std::sync::atomic::Ordering::SeqCst);
        
        self.sample_counter.clone()
    }
}

#[inline]
fn update_playback_position(prod: &mut BufferProducer, sample_n: SampleType, info: &cpal::OutputCallbackInfo) {
    let os_instant = std::time::Instant::now();
    let latency = info.timestamp().playback.duration_since(&info.timestamp().callback);

    prod.update(
        SampleTimestamp::new(os_instant, sample_n)
    );
}

pub fn open<S, E>(
    device: &cpal::Device,
    config: &OutputStreamConfig,
    mut source: S,
    error_callback: E,
) -> Result<StreamHandle, StreamError>
where
    S: Source<Item = rodio::Sample> + Send + 'static,
    E: FnMut(cpal::StreamError) + Send + 'static,
{
    let (mut prod, mut cons): (BufferProducer, BufferConsumer) = crate::new_sample_timestamp_buffer(100, std::time::Instant::now());
    let sample_counter = Arc::new(AtomicU64::new(0));

    let sample_format = config.sample_format;
    let channels = config.channel_count as SampleType;
    let config = config.into();

    let mut sample_n: SampleType = 0;

    let handle = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream::<f32, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().unwrap_or(0f32))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::F64 => device.build_output_stream::<f64, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0f64))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I8 => device.build_output_stream::<i8, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i8))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream::<i16, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i16))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_output_stream::<i32, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i32))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I64 => device.build_output_stream::<i64, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i64))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::U8 => device.build_output_stream::<u8, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut().for_each(|d| {
                    *d = source
                        .next()
                        .map(Sample::from_sample)
                        .unwrap_or(u8::MAX / 2)
                })
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream::<u16, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut().for_each(|d| {
                    *d = source
                        .next()
                        .map(Sample::from_sample)
                        .unwrap_or(u16::MAX / 2)
                })
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::U32 => device.build_output_stream::<u32, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut().for_each(|d| {
                    *d = source
                        .next()
                        .map(Sample::from_sample)
                        .unwrap_or(u32::MAX / 2)
                })
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::U64 => device.build_output_stream::<u64, _, _>(
            &config,
            move |data, info| {
                update_playback_position(&mut prod, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut().for_each(|d| {
                    *d = source
                        .next()
                        .map(Sample::from_sample)
                        .unwrap_or(u64::MAX / 2)
                })
            },
            error_callback,
            None,
        ),
        _ => return Err(StreamError::UnsupportedSampleFormat),
    }
    .map_err(StreamError::BuildStreamError)?;

    // Some platforms do not start playback as soon as we create the stream.
    handle.play().map_err(StreamError::PlayStreamError)?;

    Ok(
        StreamHandle {
            _handle: handle,
            sample_counter,
            sample_timestamp_consumer: cons,
        }
    )
}
