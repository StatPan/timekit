#[cfg(test)]
mod tests {
    use timekit::*;

    // Tests for the is_leap_year function
    #[test]
    fn test_is_leap_year() {
        // Regular leap year case
        assert_eq!(is_leap_year(2024), true);   // Leap year
        // Regular non-leap year case
        assert_eq!(is_leap_year(2023), false);  // Common year
        // Leap year divisible by 400 (special case for century leap years)
        assert_eq!(is_leap_year(2000), true);   // Leap year (divisible by 400)
        // Non-leap century year divisible by 100 but not by 400
        assert_eq!(is_leap_year(1900), false);  // Common year (divisible by 100 but not by 400)
        // Edge case for year 0 (which is considered a leap year)
        assert_eq!(is_leap_year(0), true);      // Year 0 is a leap year
        // Large year case (year 4000)
        assert_eq!(is_leap_year(4000), true);   // Leap year
        // Non-leap year just before a leap year
        assert_eq!(is_leap_year(3999), false);  // Common year
    }

    // Tests for the days_in_month function
    #[test]
    fn test_days_in_month() {
        // Regular months with 31 days
        assert_eq!(days_in_month(1, 2023), 31);  // January
        assert_eq!(days_in_month(12, 2023), 31); // December
        // Regular months with 30 days
        assert_eq!(days_in_month(4, 2023), 30);  // April
        assert_eq!(days_in_month(6, 2023), 30);  // June
        // February in a leap year
        assert_eq!(days_in_month(2, 2024), 29);  // Leap year February
        // February in a common year
        assert_eq!(days_in_month(2, 2023), 28);  // Common year February
        // Edge case for invalid month
        assert_eq!(days_in_month(13, 2023), 0);  // Invalid month (month 13)
        assert_eq!(days_in_month(0, 2023), 0);   // Invalid month (month 0)
    }

    // Tests for the now function with different timezones
    #[test]
    fn test_now_utc() {
        let current_time_utc = now(TimeZone::UTC);
        // Ensure year is valid and not before the Unix epoch
        assert!(current_time_utc.year >= 1970); // Year must be after 1970
        // Ensure month is valid
        assert!(current_time_utc.month >= 1 && current_time_utc.month <= 12); // Month range 1-12
        // Ensure day is valid
        assert!(current_time_utc.day >= 1 && current_time_utc.day <= 31); // Day range 1-31
    }

    #[test]
    fn test_now_kst() {
        let current_time_kst = now(TimeZone::KST);
        // Ensure hour is within the valid range for KST (0-23)
        assert!(current_time_kst.hour <= 23); // Removed unnecessary >= 0 check for u64
        // Ensure the day and hour values are logically consistent (no hour overflow)
        assert!(current_time_kst.day >= 1 && current_time_kst.day <= 31); // Day range 1-31
    }

    // Edge case: Test transition from December 31st to January 1st in different timezonesㅁ
    #[test]
    fn test_kst_year_boundary_transition() {
        // UTC time on December 31, 2023, 23:59:59
        let december_31_utc = DateTime::new(2023, 12, 31, 23, 59, 59);
        
        // Expected KST time should be January 1, 2024, 08:59:59 (KST is +9 hours ahead)
        let january_1_kst = DateTime::new(2024, 1, 1, 8, 59, 59);
    
        // Calculate KST time by adding 9 hours
        let mut kst_hour = december_31_utc.hour + 9;
        let mut kst_day = december_31_utc.day;
        let mut kst_month = december_31_utc.month;
        let mut kst_year = december_31_utc.year;
        let minute = december_31_utc.minute;
        let second = december_31_utc.second;
    
        // Adjust if the hour exceeds 24
        if kst_hour >= 24 {
            kst_hour -= 24;
            kst_day += 1;
        }
    
        // Adjust day and month if needed (handle month overflow)
        let days_in_current_month = days_in_month(kst_month, kst_year);
        if kst_day > days_in_current_month {
            kst_day = 1;
            kst_month += 1;
            if kst_month > 12 {
                kst_month = 1;
                kst_year += 1;
            }
        }
    
        // Verify that the calculated KST time matches the expected time
        assert_eq!(january_1_kst.hour, kst_hour);
        assert_eq!(january_1_kst.day, kst_day);
        assert_eq!(january_1_kst.month, kst_month);
        assert_eq!(january_1_kst.year, kst_year);
        assert_eq!(january_1_kst.minute, minute);
        assert_eq!(january_1_kst.second, second);
    }

    #[test]
    fn test_ist_year_boundary_transition() {
        // UTC time on December 31, 2023, 23:59:59
        let december_31_utc = DateTime::new(2023, 12, 31, 23, 59, 59);
        
        // Expected IST time should be January 1, 2024, 05:29:59 (IST is +5 hours 30 minutes ahead)
        let january_1_ist = DateTime::new(2024, 1, 1, 5, 29, 59);
    
        // Calculate IST time by adding 5 hours and 30 minutes
        let total_minutes = december_31_utc.minute + 30;
        let ist_minute = total_minutes % 60;
        let carry_over_hour = total_minutes / 60;
        let mut ist_hour = december_31_utc.hour + 5 + carry_over_hour;
        let mut ist_day = december_31_utc.day;
        let mut ist_month = december_31_utc.month;
        let mut ist_year = december_31_utc.year;
        let second = december_31_utc.second;
    
        // Adjust if the hour exceeds 24
        if ist_hour >= 24 {
            ist_hour -= 24;
            ist_day += 1;
        }
    
        // Adjust day and month if needed (handle month overflow)
        let days_in_current_month = days_in_month(ist_month, ist_year);
        if ist_day > days_in_current_month {
            ist_day = 1;
            ist_month += 1;
            if ist_month > 12 {
                ist_month = 1;
                ist_year += 1;
            }
        }
    
        // Verify that the calculated IST time matches the expected time
        assert_eq!(january_1_ist.hour, ist_hour);
        assert_eq!(january_1_ist.minute, ist_minute);
        assert_eq!(january_1_ist.day, ist_day);
        assert_eq!(january_1_ist.month, ist_month);
        assert_eq!(january_1_ist.year, ist_year);
        assert_eq!(january_1_ist.second, second);
    }

    #[test]
    fn test_leap_year_transition_kst() {
        // UTC time on February 28, 2024, 23:59:59
        let february_28_utc = DateTime::new(2024, 2, 28, 23, 59, 59);
        
        // Expected KST time should be February 29, 2024, 08:59:59
        let february_29_kst = DateTime::new(2024, 2, 29, 8, 59, 59);
    
        // Calculate KST time by adding 9 hours
        let mut kst_hour = february_28_utc.hour + 9;
        let mut kst_day = february_28_utc.day;
        let mut kst_month = february_28_utc.month;
        let mut kst_year = february_28_utc.year;
        let minute = february_28_utc.minute;
        let second = february_28_utc.second;
    
        // Adjust if the hour exceeds 24
        if kst_hour >= 24 {
            kst_hour -= 24;
            kst_day += 1;
        }
    
        // Adjust day and month if needed
        let days_in_current_month = days_in_month(kst_month, kst_year);
        if kst_day > days_in_current_month {
            kst_day = 1;
            kst_month += 1;
            if kst_month > 12 {
                kst_month = 1;
                kst_year += 1;
            }
        }
    
        // Verify that the calculated KST time matches the expected time
        assert_eq!(february_29_kst.hour, kst_hour);
        assert_eq!(february_29_kst.day, kst_day);
        assert_eq!(february_29_kst.month, kst_month);
        assert_eq!(february_29_kst.year, kst_year);
        assert_eq!(february_29_kst.minute, minute);
        assert_eq!(february_29_kst.second, second);
    }

    #[test]
    fn test_non_leap_year_transition_kst() {
        // UTC time on February 28, 2023, 23:59:59
        let february_28_utc = DateTime::new(2023, 2, 28, 23, 59, 59);
        
        // Expected KST time should be March 1, 2023, 08:59:59
        let march_1_kst = DateTime::new(2023, 3, 1, 8, 59, 59);
    
        // Calculate KST time by adding 9 hours
        let mut kst_hour = february_28_utc.hour + 9;
        let mut kst_day = february_28_utc.day;
        let mut kst_month = february_28_utc.month;
        let mut kst_year = february_28_utc.year;
        let minute = february_28_utc.minute;
        let second = february_28_utc.second;
    
        // Adjust if the hour exceeds 24
        if kst_hour >= 24 {
            kst_hour -= 24;
            kst_day += 1;
        }
    
        // Adjust day and month if needed
        let days_in_current_month = days_in_month(kst_month, kst_year);
        if kst_day > days_in_current_month {
            kst_day = 1;
            kst_month += 1;
            if kst_month > 12 {
                kst_month = 1;
                kst_year += 1;
            }
        }
    
        // Verify that the calculated KST time matches the expected time
        assert_eq!(march_1_kst.hour, kst_hour);
        assert_eq!(march_1_kst.day, kst_day);
        assert_eq!(march_1_kst.month, kst_month);
        assert_eq!(march_1_kst.year, kst_year);
        assert_eq!(march_1_kst.minute, minute);
        assert_eq!(march_1_kst.second, second);
    }

    #[test]
    fn test_month_boundary_transition_kst() {
        // UTC time on April 30, 2024, 23:59:59
        let april_30_utc = DateTime::new(2024, 4, 30, 23, 59, 59);
        
        // Expected KST time should be May 1, 2024, 08:59:59
        let may_1_kst = DateTime::new(2024, 5, 1, 8, 59, 59);
    
        // Calculate KST time by adding 9 hours
        let mut kst_hour = april_30_utc.hour + 9;
        let mut kst_day = april_30_utc.day;
        let mut kst_month = april_30_utc.month;
        let mut kst_year = april_30_utc.year;
        let minute = april_30_utc.minute;
        let second = april_30_utc.second;
    
        // Adjust if the hour exceeds 24
        if kst_hour >= 24 {
            kst_hour -= 24;
            kst_day += 1;
        }
    
        // Adjust day and month if needed
        let days_in_current_month = days_in_month(kst_month, kst_year);
        if kst_day > days_in_current_month {
            kst_day = 1;
            kst_month += 1;
            if kst_month > 12 {
                kst_month = 1;
                kst_year += 1;
            }
        }
    
        // Verify that the calculated KST time matches the expected time
        assert_eq!(may_1_kst.hour, kst_hour);
        assert_eq!(may_1_kst.day, kst_day);
        assert_eq!(may_1_kst.month, kst_month);
        assert_eq!(may_1_kst.year, kst_year);
        assert_eq!(may_1_kst.minute, minute);
        assert_eq!(may_1_kst.second, second);
    }

    #[test]
    fn test_acst_year_boundary_transition() {
        // UTC time on December 31, 2023, 23:59:59
        let december_31_utc = DateTime::new(2023, 12, 31, 23, 59, 59);
        
        // Expected ACST time should be January 1, 2024, 09:29:59 (ACST is +9 hours 30 minutes ahead)
        let january_1_acst = DateTime::new(2024, 1, 1, 9, 29, 59);
    
        // Calculate ACST time by adding 9 hours and 30 minutes
        let total_minutes = december_31_utc.minute + 30;
        let acst_minute = total_minutes % 60;
        let carry_over_hour = total_minutes / 60;
        let mut acst_hour = december_31_utc.hour + 9 + carry_over_hour;
        let mut acst_day = december_31_utc.day;
        let mut acst_month = december_31_utc.month;
        let mut acst_year = december_31_utc.year;
        let second = december_31_utc.second;
    
        // Adjust if the hour exceeds 24
        if acst_hour >= 24 {
            acst_hour -= 24;
            acst_day += 1;
        }
    
        // Adjust day and month if needed
        let days_in_current_month = days_in_month(acst_month, acst_year);
        if acst_day > days_in_current_month {
            acst_day = 1;
            acst_month += 1;
            if acst_month > 12 {
                acst_month = 1;
                acst_year += 1;
            }
        }
    
        // Verify that the calculated ACST time matches the expected time
        assert_eq!(january_1_acst.hour, acst_hour);
        assert_eq!(january_1_acst.minute, acst_minute);
        assert_eq!(january_1_acst.day, acst_day);
        assert_eq!(january_1_acst.month, acst_month);
        assert_eq!(january_1_acst.year, acst_year);
        assert_eq!(january_1_acst.second, second);
    }

    // Test time zone transition across a large time difference
    #[test]
    fn test_timezone_conversion_large_diff() {
        let utc_time = now(TimeZone::UTC);
        let hst_time = now(TimeZone::HST); // Hawaii Standard Time is UTC -10 hours

        // Calculate the expected hour difference
        let hour_diff = if utc_time.hour >= hst_time.hour {
            utc_time.hour - hst_time.hour
        } else {
            24 + utc_time.hour - hst_time.hour
        };

        assert_eq!(hour_diff, 10); // UTC should be 10 hours ahead of HST
    }

    // Test a leap second case (even though Rust does not handle leap seconds directly)
    #[test]
    fn test_leap_second() {
        let leap_second_case = DateTime::new(2023, 12, 31, 23, 59, 60); // Simulated leap second
        assert!(leap_second_case.second <= 60); // Normally seconds range from 0-59
    }
}