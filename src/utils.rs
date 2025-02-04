use chrono::{DateTime, NaiveDate, Utc};

/// Convert a u64 Windows 100 nanosecond timestamp to a chrono DateTime
///
pub fn u64_to_datetime(timestamp_u64: u64) -> DateTime<Utc> {
    let nanos = (timestamp_u64 / 10) as i64; // Convert to microseconds
    let base_time = NaiveDate::from_ymd_opt(1601, 1, 1)
        .unwrap()
        .and_hms_nano_opt(0, 0, 0, 0)
        .unwrap();
    
    let datetime = base_time + chrono::Duration::microseconds(nanos);
    DateTime::from_naive_utc_and_offset(datetime, Utc)
}