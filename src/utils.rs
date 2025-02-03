use chrono::{DateTime, NaiveDateTime, Utc};

/// Convert a u64 Windows 100 nanosecond timestamp to a chrono DateTime
///
pub fn u64_to_datetime(timestamp_u64: u64) -> DateTime<Utc> {
    // Create base Windows epoch time (1601-01-01)
    let naive = NaiveDateTime::from_timestamp_opt(
        -11644473600, // Seconds between 1601 and 1970
        0
    ).unwrap();

    // Add the timestamp microseconds
    let naive = naive + chrono::Duration::microseconds(
        (timestamp_u64 / 10) as i64
    );

    // Convert to UTC DateTime
    DateTime::from_naive_utc_and_offset(naive, Utc)
}