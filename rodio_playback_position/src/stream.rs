use std::time::Instant;

use rtsan_standalone::nonblocking;

use rodio::Source;

use cpal::Sample;
use cpal::traits::{DeviceTrait, StreamTrait};

use crate::{
    BufferConsumer, BufferProducer, OutputStreamConfig, SampleTimestamp, SampleType, StreamError,
};

/// A handle for accessing and communicating with the audio stream.
///
/// If this struct is dropped, playback will stop.
pub struct StreamHandle {
    _handle: cpal::Stream,
    config: OutputStreamConfig,
    start_time: Instant,
    last_sample_number: SampleType,
    sample_timestamp_consumer: BufferConsumer,
}

impl StreamHandle {
    /// Returns an interpolated value for the high-precision sample counter tracking this stream.
    pub fn sample_count(&mut self) -> u64 {
        let now = Instant::now();
        let timestamp_now = now.duration_since(self.start_time);

        let timestamp_data = self.sample_timestamp_consumer.newest();

        let sample_n = timestamp_data.interpolate(timestamp_now, self.config.sample_rate);

        // Make sure samples are always increasing to prevent jitter.
        if self.last_sample_number > sample_n {
            self.last_sample_number = sample_n;
        }

        self.last_sample_number
    }
}

#[inline]
#[nonblocking]
fn update_playback_position(
    prod: &mut BufferProducer,
    start_time: Instant,
    sample_n: SampleType,
    info: &cpal::OutputCallbackInfo,
) {
    let now = Instant::now();
    let timestamp_now = now.duration_since(start_time);

    let latency = info
        .timestamp()
        .playback
        .duration_since(&info.timestamp().callback)
        .unwrap_or_default();

    prod.update(SampleTimestamp::new(timestamp_now, latency, sample_n));
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
    let start_time = Instant::now();

    let (mut prod, cons): (BufferProducer, BufferConsumer) = crate::new_sample_timestamp_buffer();

    let sample_format = config.sample_format;
    let channels = config.channel_count as SampleType;
    let _config: cpal::StreamConfig = config.into();

    let mut sample_n: SampleType = 0;

    let handle = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream::<f32, _, _>(
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().unwrap_or(0f32))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::F64 => device.build_output_stream::<f64, _, _>(
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0f64))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I8 => device.build_output_stream::<i8, _, _>(
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i8))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream::<i16, _, _>(
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i16))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_output_stream::<i32, _, _>(
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i32))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I64 => device.build_output_stream::<i64, _, _>(
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
                sample_n += data.len() as SampleType / channels;

                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i64))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::U8 => device.build_output_stream::<u8, _, _>(
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
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
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
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
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
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
            &_config,
            move |data, info| {
                update_playback_position(&mut prod, start_time, sample_n, info);
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

    Ok(StreamHandle {
        _handle: handle,
        config: config.to_owned(),
        start_time,
        last_sample_number: 0,
        sample_timestamp_consumer: cons,
    })
}
