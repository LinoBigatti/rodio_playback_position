use std::{error, fmt};

/// Errors that might occur when interfacing with audio output.
#[derive(Debug)]
pub enum StreamError {
    /// Could not start playing the stream, see [cpal::PlayStreamError] for
    /// details.
    PlayStreamError(cpal::PlayStreamError),
    /// Failed to get the stream config for the given device. See
    /// [cpal::DefaultStreamConfigError] for details.
    DefaultStreamConfigError(cpal::DefaultStreamConfigError),
    /// Error opening stream with OS. See [cpal::BuildStreamError] for details.
    BuildStreamError(cpal::BuildStreamError),
    /// Could not list supported stream configs for the device. Maybe it
    /// disconnected. For details see: [cpal::SupportedStreamConfigsError].
    SupportedStreamConfigsError(cpal::SupportedStreamConfigsError),
    /// Could not find any output device
    NoDevice,
    /// New cpal sample format that rodio does not yet support please open
    /// an issue if you run into this.
    UnsupportedSampleFormat,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PlayStreamError(e) => e.fmt(f),
            Self::BuildStreamError(e) => e.fmt(f),
            Self::DefaultStreamConfigError(e) => e.fmt(f),
            Self::SupportedStreamConfigsError(e) => e.fmt(f),
            Self::NoDevice => write!(f, "NoDevice"),
            Self::UnsupportedSampleFormat => write!(f, "UnsupportedSampleFormat"),
        }
    }
}

impl error::Error for StreamError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::PlayStreamError(e) => Some(e),
            Self::BuildStreamError(e) => Some(e),
            Self::DefaultStreamConfigError(e) => Some(e),
            Self::SupportedStreamConfigsError(e) => Some(e),
            Self::NoDevice => None,
            Self::UnsupportedSampleFormat => None,
        }
    }
}
