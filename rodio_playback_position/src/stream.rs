use rodio::Source;

use cpal::{Sample, OutputCallbackInfo};
use cpal::traits::DeviceTrait;

use crate::OutputStreamConfig;
use crate::SampleCounter;
use crate::StreamError;

pub fn open<S, E>(
    device: &cpal::Device,
    config: &OutputStreamConfig,
    mut source: S,
    error_callback: E,
) -> Result<(cpal::Stream, SampleCounter), StreamError>
where
    S: Source<Item = rodio::Sample> + Send + 'static,
    E: FnMut(cpal::StreamError) + Send + 'static,
{
    open_with_sample_counter(device, config, source, error_callback)
}
pub fn open<S, E>(
    device: &cpal::Device,
    config: &OutputStreamConfig,
    mut source: S,
    error_callback: E,
) -> Result<(cpal::Stream, SampleCounter), StreamError>
where
    S: Source<Item = rodio::Sample> + Send + 'static,
    E: FnMut(cpal::StreamError) + Send + 'static,
{
    let sample_format = config.sample_format;
    let config = config.into();

    match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream::<f32, _, _>(
            &config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().unwrap_or(0f32))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::F64 => device.build_output_stream::<f64, _, _>(
            &config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0f64))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I8 => device.build_output_stream::<i8, _, _>(
            &config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i8))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream::<i16, _, _>(
            &config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i16))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_output_stream::<i32, _, _>(
            &config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i32))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::I64 => device.build_output_stream::<i64, _, _>(
            &config,
            move |data, _| {
                data.iter_mut()
                    .for_each(|d| *d = source.next().map(Sample::from_sample).unwrap_or(0i64))
            },
            error_callback,
            None,
        ),
        cpal::SampleFormat::U8 => device.build_output_stream::<u8, _, _>(
            &config,
            move |data, _| {
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
            move |data, _| {
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
            move |data, _| {
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
            move |data, _| {
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
    .map_err(StreamError::BuildStreamError)
}
