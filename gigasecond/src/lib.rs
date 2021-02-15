
use chrono::{DateTime, Utc};
use chrono::Duration;
const GIGASECOND: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // use const for GIGASECOND
    start + Duration::seconds(GIGASECOND)
}
