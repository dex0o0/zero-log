use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// Convert unix time to UTC date time
///
/// ```rust
/// use zero_log::time_date::DTime;
///
/// let time = 1700000000;
/// let dt = DTime::from_unix(time);
/// let sdt = format!("{}",dt);
/// assert_eq!(sdt,"2023-11-14 22:13:20");
/// ```
impl DTime {
    pub fn now() -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap();

        Self::from_unix(ts)
    }
    pub fn from_unix(timestamp: i64) -> Self {
        let days = timestamp.div_euclid(86400);
        let secs_in_day = timestamp.rem_euclid(86400);

        let hour = (secs_in_day / 3600) as u8;
        let minute = ((secs_in_day % 3600) / 60) as u8;
        let second = (secs_in_day % 60) as u8;

        let z = days + 719_468;
        let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
        let doe = (z - era * 146_097) as u64;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;

        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;

        let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
        let month = if mp < 10 {
            (mp + 3) as u8
        } else {
            (mp - 9) as u8
        };

        let year = (y + if month <= 2 { 1 } else { 0 }) as i32;

        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl fmt::Display for DTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

#[test]
fn test_format_time_date() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let dt = DTime::from_unix(timestamp as i64);

    println!("{}", dt);
}
