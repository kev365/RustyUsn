use chrono::{DateTime, Utc};

/// Convert a u64 Windows 100 nanosecond timestamp to a chrono DateTime
///
pub fn u64_to_datetime(timestamp_u64: u64) -> DateTime<Utc> {
    // Convert Windows timestamp (100ns since 1601) to Unix timestamp (seconds since 1970)
    let unix_timestamp = ((timestamp_u64 / 10_000_000) as i64) - 11_644_473_600;
    let nanos = ((timestamp_u64 % 10_000_000) * 100) as u32;
    
    // Use the new from_timestamp function
    DateTime::from_timestamp(unix_timestamp, nanos)
        .unwrap_or_else(|| DateTime::<Utc>::UNIX_EPOCH)
}