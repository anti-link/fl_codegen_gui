use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

const CODE_GRANULARITY_MS: u64 = 1000 * 60;

pub fn generate_fl_code(shared_secret: String, custom_timestamp: u64) -> String {
    let key = shared_secret.trim();

    let timestamp_ms: u64 = custom_timestamp * 1000;

    let code_validity_ms = 1000u64 * 60 * 60; // 1 hour
    let interval = timestamp_ms / code_validity_ms;
    let interval_beginning_timestamp_ms = interval * code_validity_ms;
    let adjusted_timestamp = interval_beginning_timestamp_ms / CODE_GRANULARITY_MS;
    let big_endian_timestamp = adjusted_timestamp.to_be_bytes();

    let mut mac = HmacSha1::new_from_slice(key.as_bytes()).unwrap();
    mac.update(&big_endian_timestamp[..]);
    let digest = mac.finalize().into_bytes();

    let offset = (digest.last().expect("digest is not empty") & 0xf) as usize;
    // read as i32 and clear sign bit
    let result = i32::from_be_bytes(digest[offset..][..4].try_into().unwrap()) & 0x7fffffff;

    let valid_from_ms = interval_beginning_timestamp_ms;
    let valid_to_ms = valid_from_ms + code_validity_ms;

    let code: i32 = result % 1000000;
    format!("{:06}", code)
}

pub fn remaining_time(custom_timestamp: u64) -> u64 {
    let timestamp_ms: u64 = custom_timestamp * 1000;

    let code_validity_ms = 1000u64 * 60 * 60; // 1 hour
    let interval = timestamp_ms / code_validity_ms;
    let interval_beginning_timestamp_ms = interval * code_validity_ms;

    let valid_from_ms = interval_beginning_timestamp_ms;
    let valid_to_ms = valid_from_ms + code_validity_ms;

    let remaining_sec = (valid_to_ms - timestamp_ms) / 1000;

    remaining_sec
}

pub fn float_from_time(seconds_left: u64) -> f32 {
    let hours_left = seconds_left as f32 / 3600.0;
    hours_left
}
