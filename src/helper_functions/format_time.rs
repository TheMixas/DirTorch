use chrono::{DateTime, TimeZone, Utc};
use std::time::SystemTime;

pub fn format_time(system_time: &SystemTime) -> String {
    let datetime: DateTime<Utc> = (*system_time).into();
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    formatted_datetime
}