// Bring in the constants from const.rs
mod constants;
use constants::*;  // Use all constants

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Struct for holding the full date and time information.
#[derive(Debug)]
pub struct DateTime {
    pub year: u64,
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
}

impl DateTime {
    /// Creates a new `DateTime` object.
    pub fn new(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64) -> Self {
        Self { year, month, day, hour, minute, second }
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

/// Enum for representing time zones with precomputed UTC offsets in seconds.
#[derive(Debug, Clone, Copy)]
pub enum TimeZone {
    UTC,
    KST,  // Korea Standard Time (UTC+9)
    EST,  // Eastern Standard Time (UTC-5)
    PST,  // Pacific Standard Time (UTC-8)
    JST,  // Japan Standard Time (UTC+9)
    IST,  // India Standard Time (UTC+5:30)
    CET,  // Central European Time (UTC+1)
    AST,  // Atlantic Standard Time (UTC-4)
    CST,  // Central Standard Time (UTC-6)
    MST,  // Mountain Standard Time (UTC-7)
    AKST, // Alaska Standard Time (UTC-9)
    HST,  // Hawaii Standard Time (UTC-10)
    BST,  // British Summer Time (UTC+1)
    WET,  // Western European Time (UTC+0)
    EET,  // Eastern European Time (UTC+2)
    SAST, // South Africa Standard Time (UTC+2)
    EAT,  // East Africa Time (UTC+3)
    AEST, // Australian Eastern Standard Time (UTC+10)
    ACST, // Australian Central Standard Time (UTC+9:30)
    AWST, // Australian Western Standard Time (UTC+8)
    CSTAsia,  // China Standard Time (UTC+8)
    SGT,  // Singapore Time (UTC+8)
    HKT,  // Hong Kong Time (UTC+8)
}

impl TimeZone {
    /// Returns the precomputed UTC offset in seconds for each time zone.
    pub fn offset_in_seconds(&self) -> i64 {
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
pub fn now(timezone: TimeZone) -> DateTime {
    // Get the current system time since UNIX_EPOCH in seconds and milliseconds.
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    
    // Total seconds since UNIX epoch.
    let total_seconds = duration_since_epoch.as_secs();
    
    // Get the time zone offset in seconds.
    let timezone_offset = timezone.offset_in_seconds();
    
    // Adjust total seconds based on the time zone offset.
    let adjusted_seconds = (total_seconds as i64 + timezone_offset) as u64;

    // Convert adjusted seconds into days, hours, minutes, and seconds.
    let mut days = adjusted_seconds / SECONDS_IN_DAY;
    let remainder_seconds = adjusted_seconds % SECONDS_IN_DAY;
    let hours = remainder_seconds / SECONDS_IN_HOUR;
    let remainder_seconds = remainder_seconds % SECONDS_IN_HOUR;
    let minutes = remainder_seconds / SECONDS_IN_MINUTE;
    let seconds = remainder_seconds % SECONDS_IN_MINUTE;

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
    let day = days + 1; // Days start from 1.

    // Return the DateTime object.
    DateTime::new(year, month, day, hours, minutes, seconds)
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
pub fn is_leap_year(year: u64) -> bool {
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
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,  // January, March, May, July, August, October, December have 31 days
        4 | 6 | 9 | 11 => 30,  // April, June, September, November have 30 days
        2 => {
            // February has 29 days in a leap year, otherwise it has 28 days
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,  // Invalid month input, returns 0 (shouldn't happen with proper input validation)
    }
}