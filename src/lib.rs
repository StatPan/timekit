// Bring in the constants from const.rs
pub mod constants;

use constants::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fmt, ops::Add, ops::Sub};

/// Struct for holding the full date and time information.
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub year: u64,
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
    pub timezone: TimeZone,
}

impl DateTime {
    /// Creates a new `DateTime` object.
    pub fn new(
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
        timezone: TimeZone,
    ) -> Result<Self, String> {
        if month < 1 || month > 12 {
            return Err("Invalid month".to_string());
        }
        if day < 1 || day > days_in_month(month, year) {
            return Err("Invalid day".to_string());
        }
        if hour > 23 {
            return Err("Invalid hour".to_string());
        }
        if minute > 59 {
            return Err("Invalid minute".to_string());
        }
        if second > 59 {
            return Err("Invalid second".to_string());
        }

        // Unix 초를 계산하기 위해 UTC 시간 기준으로 보정
        let mut total_seconds = 0;

        // 연도 계산
        for y in 1970..year {
            total_seconds += if is_leap_year(y) { 366 } else { 365 } * SECONDS_IN_DAY;
        }

        // 월 계산
        for m in 1..month {
            total_seconds += days_in_month(m, year) as i64 * SECONDS_IN_DAY;
        }

        // 일, 시, 분, 초 계산
        total_seconds += (day - 1) as i64 * SECONDS_IN_DAY;
        total_seconds += hour as i64 * SECONDS_IN_HOUR;
        total_seconds += minute as i64 * SECONDS_IN_MINUTE;
        total_seconds += second as i64;

        // 시간대 오프셋 적용 (UTC 기준으로 보정)
        total_seconds -= timezone.offset_in_seconds();

        // UTC 기준의 `DateTime` 생성
        let utc_datetime = Self::from_unix_seconds(total_seconds, timezone)?;

        Ok(utc_datetime)
    }

    /// Calculate the total seconds since Unix Epoch (1970-01-01 00:00:00)
    pub fn calculate_total_seconds(
        year: u64,
        month: u64,
        day: u64,
        hour: u64,
        minute: u64,
        second: u64,
    ) -> Result<i64, String> {
        // 연도는 1970년부터 시작
        if year < 1970 {
            return Err("Year must be 1970 or later".to_string());
        }

        let mut total_seconds: i64 = 0;

        // 1. 연도 계산: 1970년부터 현재 연도 이전까지의 총 초 계산
        for y in 1970..year {
            total_seconds += if is_leap_year(y) {
                SECONDS_IN_LEAPYEAR
            } else {
                SECONDS_IN_YEAR
            };
        }

        // 2. 월 계산: 1월부터 현재 월 이전까지의 총 초 계산
        for m in 1..month {
            total_seconds += days_in_month(m, year) as i64 * SECONDS_IN_DAY;
        }

        // 3. 일 계산: 현재 월에서 1일부터 현재 일 이전까지의 총 초 계산
        total_seconds += (day as i64 - 1) * SECONDS_IN_DAY;

        // 4. 시, 분, 초 계산
        total_seconds += hour as i64 * SECONDS_IN_HOUR;
        total_seconds += minute as i64 * SECONDS_IN_MINUTE;
        total_seconds += second as i64;

        Ok(total_seconds)
    }

    pub fn strftime(&self, format: &str) -> String {
        let mut result = format.to_string();
        result = result.replace("%Y", &format!("{:04}", self.year));
        result = result.replace("%m", &format!("{:02}", self.month));
        result = result.replace("%d", &format!("{:02}", self.day));
        result = result.replace("%H", &format!("{:02}", self.hour));
        result = result.replace("%M", &format!("{:02}", self.minute));
        result = result.replace("%S", &format!("{:02}", self.second));
        result
    }

    pub fn to_unix_seconds(&self) -> i64 {
        let mut total_seconds: i64 = 0;

        // 연도 계산
        for year in 1970..self.year {
            total_seconds += if is_leap_year(year) { 366 } else { 365 } * SECONDS_IN_DAY;
        }

        // 월 계산
        for month in 1..self.month {
            total_seconds += days_in_month(month, self.year) as i64 * SECONDS_IN_DAY;
        }

        // 일, 시, 분, 초 계산
        total_seconds += (self.day - 1) as i64 * SECONDS_IN_DAY;
        total_seconds += self.hour as i64 * SECONDS_IN_HOUR;
        total_seconds += self.minute as i64 * SECONDS_IN_MINUTE;
        total_seconds += self.second as i64;

        // 시간대 오프셋 적용 (UTC 기준으로 변환)
        total_seconds - self.timezone.offset_in_seconds()
    }

    pub fn from_unix_seconds(unix_seconds: i64, timezone: TimeZone) -> Result<Self, String> {
        // 시간대 오프셋 적용 (UTC에서 로컬 시간으로 변환)
        let adjusted_seconds = unix_seconds + timezone.offset_in_seconds();
        let mut remaining_seconds = adjusted_seconds;

        if remaining_seconds < 0 {
            return Err("Unix seconds cannot represent a date before 1970-01-01".to_string());
        }

        // 연도 계산
        let mut year = 1970;
        while remaining_seconds >= (if is_leap_year(year) { 366 } else { 365 }) * SECONDS_IN_DAY {
            remaining_seconds -= (if is_leap_year(year) { 366 } else { 365 }) * SECONDS_IN_DAY;
            year += 1;
        }

        // 월 계산
        let mut month = 1;
        while remaining_seconds >= days_in_month(month, year) as i64 * SECONDS_IN_DAY {
            remaining_seconds -= days_in_month(month, year) as i64 * SECONDS_IN_DAY;
            month += 1;
        }

        // 일, 시, 분, 초 계산
        let day = (remaining_seconds / SECONDS_IN_DAY) as u64 + 1;
        remaining_seconds %= SECONDS_IN_DAY;
        let hour = (remaining_seconds / 3600) as u64;
        remaining_seconds %= 3600;
        let minute = (remaining_seconds / 60) as u64;
        let second = (remaining_seconds % 60) as u64;

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            timezone,
        })
    }

    pub fn add_timedelta(&self, delta: TimeDelta) -> Result<Self, String> {
        let current_unix = self.to_unix_seconds(); // 현재 시간을 Unix 시간으로 변환
        let delta_seconds = compute_total_seconds(
            delta.weeks,
            delta.days,
            delta.hours,
            delta.minutes,
            delta.seconds,
        );
        let timezone = self.timezone.clone();
        let new_unix = current_unix + delta_seconds; // 초 단위로 더하기
        if new_unix < 0 {
            return Err(
                "Resulting DateTime is before Unix epoch (1970-01-01 00:00:00 UTC)".to_string(),
            );
        }
        DateTime::from_unix_seconds(new_unix, timezone) // 다시 DateTime으로 변환
    }

    pub fn sub_timedelta(&self, delta: TimeDelta) -> Result<Self, String> {
        let current_unix = self.to_unix_seconds(); // 현재 시간을 Unix 시간으로 변환
        let delta_seconds = compute_total_seconds(
            delta.weeks,
            delta.days,
            delta.hours,
            delta.minutes,
            delta.seconds,
        );
        let new_unix = current_unix - delta_seconds; // 초 단위로 빼기
        let timezone = self.timezone.clone();
        if new_unix < 0 {
            return Err(
                "Resulting DateTime is before Unix epoch (1970-01-01 00:00:00 UTC)".to_string(),
            );
        }

        DateTime::from_unix_seconds(new_unix, timezone) // 다시 DateTime으로 변환
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the DateTime struct to a readable "YYYY-MM-DD HH:MM:SS" format.
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

impl Add<TimeDelta> for DateTime {
    type Output = Result<DateTime, String>;

    fn add(self, delta: TimeDelta) -> Self::Output {
        DateTime::add_timedelta(&self, delta)
    }
}

impl Sub<TimeDelta> for DateTime {
    type Output = Result<DateTime, String>;

    fn sub(self, delta: TimeDelta) -> Self::Output {
        DateTime::sub_timedelta(&self, delta)
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
            && self.second == other.second
    }
}

/// TimeDelta struct to represent a time difference similar to Python's timedelta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeDelta {
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl std::fmt::Display for TimeDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut components = Vec::new();

        if self.weeks != 0 {
            components.push(format!(
                "{} week{}",
                self.weeks,
                if self.weeks.abs() == 1 { "" } else { "s" }
            ));
        }
        if self.days != 0 {
            components.push(format!(
                "{} day{}",
                self.days,
                if self.days.abs() == 1 { "" } else { "s" }
            ));
        }
        if self.hours != 0 {
            components.push(format!(
                "{} hour{}",
                self.hours,
                if self.hours.abs() == 1 { "" } else { "s" }
            ));
        }
        if self.minutes != 0 {
            components.push(format!(
                "{} minute{}",
                self.minutes,
                if self.minutes.abs() == 1 { "" } else { "s" }
            ));
        }
        if self.seconds != 0 || components.is_empty() {
            // 항상 seconds는 출력
            components.push(format!(
                "{} second{}",
                self.seconds,
                if self.seconds.abs() == 1 { "" } else { "s" }
            ));
        }

        write!(f, "{}", components.join(", "))
    }
}

impl Default for TimeDelta {
    fn default() -> Self {
        Self {
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

/// Enum for representing time zones with precomputed UTC offsets in seconds.
#[derive(Debug, Clone, Copy)]
pub enum TimeZone {
    UTC,
    KST,     // Korea Standard Time (UTC+9)
    EST,     // Eastern Standard Time (UTC-5)
    PST,     // Pacific Standard Time (UTC-8)
    JST,     // Japan Standard Time (UTC+9)
    IST,     // India Standard Time (UTC+5:30)
    CET,     // Central European Time (UTC+1)
    AST,     // Atlantic Standard Time (UTC-4)
    CST,     // Central Standard Time (UTC-6)
    MST,     // Mountain Standard Time (UTC-7)
    AKST,    // Alaska Standard Time (UTC-9)
    HST,     // Hawaii Standard Time (UTC-10)
    BST,     // British Summer Time (UTC+1)
    WET,     // Western European Time (UTC+0)
    EET,     // Eastern European Time (UTC+2)
    SAST,    // South Africa Standard Time (UTC+2)
    EAT,     // East Africa Time (UTC+3)
    AEST,    // Australian Eastern Standard Time (UTC+10)
    ACST,    // Australian Central Standard Time (UTC+9:30)
    AWST,    // Australian Western Standard Time (UTC+8)
    CSTAsia, // China Standard Time (UTC+8)
    SGT,     // Singapore Time (UTC+8)
    HKT,     // Hong Kong Time (UTC+8)
}

impl TimeZone {
    /// Returns the precomputed UTC offset in seconds for each time zone.
    pub const fn offset_in_seconds(&self) -> i64 {
        match self {
            TimeZone::UTC => OFFSET_UTC,
            TimeZone::KST => OFFSET_KST,
            TimeZone::EST => OFFSET_EST,
            TimeZone::PST => OFFSET_PST,
            TimeZone::JST => OFFSET_JST,
            TimeZone::IST => OFFSET_IST,
            TimeZone::CET => OFFSET_CET,
            TimeZone::AST => OFFSET_AST,
            TimeZone::CST => OFFSET_CST,
            TimeZone::MST => OFFSET_MST,
            TimeZone::AKST => OFFSET_AKST,
            TimeZone::HST => OFFSET_HST,
            TimeZone::BST => OFFSET_BST,
            TimeZone::WET => OFFSET_WET,
            TimeZone::EET => OFFSET_EET,
            TimeZone::SAST => OFFSET_SAST,
            TimeZone::EAT => OFFSET_EAT,
            TimeZone::AEST => OFFSET_AEST,
            TimeZone::ACST => OFFSET_ACST,
            TimeZone::AWST => OFFSET_AWST,
            TimeZone::CSTAsia => OFFSET_CST_ASIA,
            TimeZone::SGT => OFFSET_SGT,
            TimeZone::HKT => OFFSET_HKT,
        }
    }
}

/// Returns the current date and time adjusted for the specified time zone.
///
/// This function calculates the current date and time based on the system's current time
/// (measured as the number of seconds since the UNIX Epoch: 1970-01-01 00:00:00 UTC)
/// and adjusts it according to the time zone provided. The time zone offsets are hardcoded
/// to avoid unnecessary runtime computation.
///
/// The function follows these steps:
/// 1. Retrieves the current system time as seconds since the UNIX Epoch.
/// 2. Applies the specified time zone's UTC offset to the seconds.
/// 3. Converts the adjusted seconds into days, hours, minutes, and seconds.
/// 4. Determines the corresponding year, month, and day using the leap year rules.
/// 5. Returns the computed time as a `DateTime` object containing the year, month, day, hour, minute, and second.
///
/// # Parameters:
/// * `timezone`: The `TimeZone` enum that specifies the time zone for which the current time should be adjusted.
///
/// # Returns:
/// * `DateTime`: A struct containing the current year, month, day, hour, minute, and second, adjusted to the specified time zone.
///
/// # Panics:
/// * The function will panic if the system's time goes backwards (i.e., if the current time is somehow earlier than the UNIX Epoch).
///
/// # Example:
/// ```
/// use timekit;
/// use timekit::TimeZone;
/// let current_time_kst = timekit::now(TimeZone::KST);  // Returns current time in Korea Standard Time (KST).
/// let current_time_utc = timekit::now(TimeZone::UTC);  // Returns current time in UTC.
/// ```
pub fn now(timezone: TimeZone) -> Result<DateTime, String> {
    // Get the current system time since UNIX_EPOCH in seconds and milliseconds.
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Total seconds since UNIX epoch.
    let total_seconds = duration_since_epoch.as_secs();

    // Get the time zone offset in seconds.
    //let timezone_offset = timezone.offset_in_seconds();
    //let adjusted_seconds = (total_seconds as i64 + timezone_offset) as u64;
    // Adjust total seconds based on the time zone offset.
    let adjusted_seconds = adjust_second_with_timezone(total_seconds, timezone);

    calculate_date_since_epoch(adjusted_seconds as i64, timezone)
}

/// Determines if a given year is a leap year.
///
/// A leap year is a year that is divisible by 4 but not divisible by 100,
/// except when the year is also divisible by 400. This rule is part of the
/// Gregorian calendar, which adds an extra day to February (29 days) to
/// keep the calendar year synchronized with the astronomical year.
///
/// # Parameters:
/// * `year`: The year as a `u64` to be checked for leap year status.
///
/// # Returns:
/// * `true` if the year is a leap year, otherwise `false`.
///
/// # Example:
/// ```
/// use timekit::is_leap_year;
/// let leap_year = is_leap_year(2024);  // true
/// let common_year = is_leap_year(2023);  // false
/// ```
pub const fn is_leap_year(year: u64) -> bool {
    // A leap year is divisible by 4 but not divisible by 100,
    // except if it is divisible by 400.
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Returns the number of days in a given month and year.
///
/// This function returns the number of days in a month. It accounts for leap years
/// in February, where there are 29 days instead of the usual 28. All other months
/// follow the standard day count:
/// - January, March, May, July, August, October, and December have 31 days.
/// - April, June, September, and November have 30 days.
/// - February has 28 days in common years and 29 days in leap years.
///
/// # Parameters:
/// * `month`: The month (1-12) as a `u64`. 1 corresponds to January, and 12 corresponds to December.
/// * `year`: The year as a `u64`. The year is needed to determine whether February has 28 or 29 days in case of a leap year.
///
/// # Returns:
/// * The number of days in the specified month as a `u64`.
///
/// # Example:
/// ```
/// use timekit::days_in_month;
/// let days_in_january = days_in_month(1, 2024);  // 31
/// let days_in_february_leap_year = days_in_month(2, 2024);  // 29
/// let days_in_february_common_year = days_in_month(2, 2023);  // 28
/// ```
pub fn days_in_month(month: u64, year: u64) -> u64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31, // January, March, May, July, August, October, December have 31 days
        4 | 6 | 9 | 11 => 30,              // April, June, September, November have 30 days
        2 => {
            // February has 29 days in a leap year, otherwise it has 28 days
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0, // Invalid month input, returns 0 (shouldn't happen with proper input validation)
    }
}

pub const fn compute_total_seconds(
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
) -> i64 {
    weeks * SECONDS_IN_WEEK as i64
        + days * SECONDS_IN_DAY as i64
        + hours * SECONDS_IN_HOUR as i64
        + minutes * SECONDS_IN_MINUTE as i64
        + seconds
}

pub const fn adjust_second_with_timezone(total_seconds: u64, timezone: TimeZone) -> u64 {
    let timezone_offset = timezone.offset_in_seconds();
    let adjusted_seconds = (total_seconds as i64 + timezone_offset) as u64;
    adjusted_seconds
}

pub fn calculate_date_since_epoch(
    adjusted_seconds: i64,
    timezone: TimeZone,
) -> Result<DateTime, String> {
    // Convert adjusted seconds into days, hours, minutes, and seconds.
    let mut days = adjusted_seconds as u64 / SECONDS_IN_DAY as u64;
    let remainder_seconds = adjusted_seconds % SECONDS_IN_DAY;
    let hour = (remainder_seconds / SECONDS_IN_HOUR) as u64;
    let remainder_seconds = remainder_seconds % SECONDS_IN_HOUR;
    let minute = (remainder_seconds / SECONDS_IN_MINUTE) as u64;
    let second = (remainder_seconds % SECONDS_IN_MINUTE) as u64;

    // Year calculation (starting from 1970).
    let mut year = 1970;
    while days >= if is_leap_year(year) { 366 } else { 365 } {
        days -= if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }

    // Month and day calculation.
    let mut month = 1;
    while days >= days_in_month(month, year) {
        days -= days_in_month(month, year);
        month += 1;
    }
    let day = days as u64 + 1; // Days start from 1.

    // Return the DateTime object.
    DateTime::new(year, month, day, hour, minute, second, timezone)
}
