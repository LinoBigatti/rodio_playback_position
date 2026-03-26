use rodio_playback_position::sample::{duration_to_samples, SampleTimestamp};
use rodio_playback_position::SampleType;

use std::time::Duration;

fn create_sample_timestamp(
    timestamp_secs: u64,
    timestamp_nanos: u32,
    latency_secs: u64,
    latency_nanos: u32,
    sample_n: SampleType,
) -> SampleTimestamp {
    SampleTimestamp::new(
        Duration::new(timestamp_secs, timestamp_nanos),
        Duration::new(latency_secs, latency_nanos),
        sample_n,
    )
}

#[test]
fn test_duration_to_samples_basic() {
    assert_eq!(duration_to_samples(Duration::from_secs(1), 44100), 44100);
    assert_eq!(duration_to_samples(Duration::from_secs(2), 48000), 96000);
}

#[test]
fn test_duration_to_samples_zero_duration() {
    assert_eq!(duration_to_samples(Duration::ZERO, 44100), 0);
    assert_eq!(duration_to_samples(Duration::ZERO, 0), 0);
}

#[test]
fn test_duration_to_samples_sub_second() {
    assert_eq!(duration_to_samples(Duration::from_millis(500), 44100), 22050);
    assert_eq!(duration_to_samples(Duration::from_millis(250), 48000), 12000);
    assert_eq!(duration_to_samples(Duration::from_nanos(1), 44100), 0);
}

#[test]
fn test_duration_to_samples_high_precision() {
    let nanos_per_sample = 1_000_000_000u32.div_ceil(44100);

    let one_sample_duration = Duration::new(0, nanos_per_sample);
    assert_eq!(duration_to_samples(one_sample_duration, 44100), 1);

    let slightly_less = Duration::new(0, nanos_per_sample - 1);
    assert_eq!(duration_to_samples(slightly_less, 44100), 0);

    let slightly_more = Duration::new(0, nanos_per_sample + 1);
    assert_eq!(duration_to_samples(slightly_more, 44100), 1);
}

#[test]
fn test_duration_to_samples_large_values() {
    let large_duration = Duration::from_secs(3600 * 24 * 365);
    let large_sample_rate = 192000;
    let expected_samples = 3600 * 24 * 365 * 192000;
    assert_eq!(duration_to_samples(large_duration, large_sample_rate), expected_samples);
}

#[test]
fn test_duration_to_samples_zero_sample_rate() {
    assert_eq!(duration_to_samples(Duration::from_secs(1), 0), 0);
    assert_eq!(duration_to_samples(Duration::from_millis(500), 0), 0);
}

#[test]
fn test_interpolate_current_timestamp_after() {
    let sample_rate = 44100;
    let initial_sample_n = 1000;
    let st = create_sample_timestamp(10, 0, 0, 0, initial_sample_n);

    let current_ts = Duration::from_secs(11);
    let expected_samples_added = duration_to_samples(Duration::from_secs(1), sample_rate);
    assert_eq!(st.interpolate(current_ts, sample_rate), initial_sample_n + expected_samples_added);
}

#[test]
fn test_interpolate_current_timestamp_before() {
    let sample_rate = 44100;
    let initial_sample_n = 100000;
    let st = create_sample_timestamp(10, 0, 0, 0, initial_sample_n);

    let current_ts = Duration::from_secs(9);
    let expected_samples_subtracted = duration_to_samples(Duration::from_secs(1), sample_rate);
    assert_eq!(st.interpolate(current_ts, sample_rate), initial_sample_n - expected_samples_subtracted);
}

#[test]
fn test_interpolate_current_timestamp_equal() {
    let sample_rate = 44100;
    let initial_sample_n = 1000;
    let st = create_sample_timestamp(10, 0, 0, 0, initial_sample_n);

    let current_ts = Duration::from_secs(10);
    assert_eq!(st.interpolate(current_ts, sample_rate), initial_sample_n);
}

#[test]
fn test_interpolate_with_latency_positive_result() {
    let sample_rate = 44100;
    let initial_sample_n = 100000;
    let latency_secs = 1;
    let st = create_sample_timestamp(10, 0, latency_secs, 0, initial_sample_n);

    let current_ts = Duration::from_secs(10);
    let expected_latency_samples = duration_to_samples(Duration::from_secs(latency_secs), sample_rate);
    assert_eq!(st.interpolate(current_ts, sample_rate), initial_sample_n - expected_latency_samples);
}

#[test]
fn test_interpolate_with_latency_zero_result() {
    let sample_rate = 44100;
    let initial_sample_n = 1000;
    let latency_secs = 1;
    let st = create_sample_timestamp(10, 0, latency_secs, 0, initial_sample_n);

    let current_ts = Duration::from_secs(10);
    assert_eq!(st.interpolate(current_ts, sample_rate), 0);
}

#[test]
fn test_interpolate_with_latency_exactly_zero_result() {
    let sample_rate = 44100;
    let latency_secs = 1;
    let expected_latency_samples = duration_to_samples(Duration::from_secs(latency_secs), sample_rate);
    let initial_sample_n = expected_latency_samples;
    let st = create_sample_timestamp(10, 0, latency_secs, 0, initial_sample_n);

    let current_ts = Duration::from_secs(10);
    assert_eq!(st.interpolate(current_ts, sample_rate), 0);
}

#[test]
fn test_interpolate_zero_sample_rate() {
    let sample_rate = 0;
    let initial_sample_n = 1000;
    let st = create_sample_timestamp(10, 0, 1, 0, initial_sample_n);

    assert_eq!(st.interpolate(Duration::from_secs(11), sample_rate), initial_sample_n);
    assert_eq!(st.interpolate(Duration::from_secs(9), sample_rate), initial_sample_n);
    assert_eq!(st.interpolate(Duration::from_secs(10), sample_rate), initial_sample_n);
}

#[test]
fn test_interpolate_complex_scenario() {
    let sample_rate = 48000;
    let initial_sample_n = 50000;
    let st_timestamp = Duration::from_secs(5);
    let latency = Duration::from_millis(500);
    let st = create_sample_timestamp(st_timestamp.as_secs(), st_timestamp.subsec_nanos(), latency.as_secs(), latency.subsec_nanos(), initial_sample_n);

    let current_ts = Duration::from_millis(5250);
    let time_diff = Duration::from_millis(250);
    let time_diff_samples = duration_to_samples(time_diff, sample_rate);

    let expected_sample_n_before_latency = initial_sample_n + time_diff_samples;
    let expected_latency_samples = duration_to_samples(latency, sample_rate);

    assert_eq!(st.interpolate(current_ts, sample_rate), expected_sample_n_before_latency - expected_latency_samples);
}

#[test]
fn test_interpolate_negative_sample_n_becomes_zero() {
    let sample_rate = 44100;
    let initial_sample_n = 1000;
    let st = create_sample_timestamp(10, 0, 0, 0, initial_sample_n);

    let current_ts = Duration::from_secs(8);
    assert_eq!(st.interpolate(current_ts, sample_rate), 0);
}

#[test]
fn test_interpolate_accuracy_sub_second() {
    let sample_rate = 44100;
    let initial_sample_n = 0;
    let st = create_sample_timestamp(0, 0, 0, 0, initial_sample_n);

    let current_ts = Duration::from_millis(500);
    let expected_samples = duration_to_samples(current_ts, sample_rate);
    assert_eq!(st.interpolate(current_ts, sample_rate), expected_samples);
}

#[test]
fn test_interpolate_accuracy_with_nanos() {
    let sample_rate = 44100;
    let initial_sample_n = 100000;
    let st = create_sample_timestamp(10, 500_000_000, 0, 0, initial_sample_n);

    let current_ts = Duration::new(10, 750_000_000);
    let time_diff = Duration::from_millis(250);
    let expected_samples_added = duration_to_samples(time_diff, sample_rate);
    assert_eq!(st.interpolate(current_ts, sample_rate), initial_sample_n + expected_samples_added);

    let current_ts_before = Duration::new(10, 250_000_000);
    let time_diff_before = Duration::from_millis(250);
    let expected_samples_subtracted = duration_to_samples(time_diff_before, sample_rate);
    assert_eq!(st.interpolate(current_ts_before, sample_rate), initial_sample_n - expected_samples_subtracted);
}
