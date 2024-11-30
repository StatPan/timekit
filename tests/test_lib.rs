#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Add, Sub};
    use timekit::{
        adjust_second_with_timezone, calculate_date_since_epoch, compute_total_seconds,
        constants::*, days_in_month, is_leap_year, now, DateTime, TimeDelta, TimeZone,
    };

    // Test the is_leap_year function
    #[test]
    fn test_is_leap_year() {
        // Regular leap years
        assert_eq!(is_leap_year(2020), true);
        assert_eq!(is_leap_year(2024), true);

        // Century years not leap years unless divisible by 400
        assert_eq!(is_leap_year(1900), false);
        assert_eq!(is_leap_year(2000), true);

        // Common years
        assert_eq!(is_leap_year(2019), false);
        assert_eq!(is_leap_year(2021), false);
    }

    // Test the days_in_month function
    #[test]
    fn test_days_in_month() {
        // Months with 31 days
        assert_eq!(days_in_month(1, 2023), 31);
        assert_eq!(days_in_month(3, 2023), 31);
        assert_eq!(days_in_month(5, 2023), 31);
        assert_eq!(days_in_month(7, 2023), 31);
        assert_eq!(days_in_month(8, 2023), 31);
        assert_eq!(days_in_month(10, 2023), 31);
        assert_eq!(days_in_month(12, 2023), 31);

        // Months with 30 days
        assert_eq!(days_in_month(4, 2023), 30);
        assert_eq!(days_in_month(6, 2023), 30);
        assert_eq!(days_in_month(9, 2023), 30);
        assert_eq!(days_in_month(11, 2023), 30);

        // February in leap year
        assert_eq!(days_in_month(2, 2024), 29);

        // February in common year
        assert_eq!(days_in_month(2, 2023), 28);

        // Invalid month
        assert_eq!(days_in_month(13, 2023), 0);
        assert_eq!(days_in_month(0, 2023), 0);
    }

    // Test the DateTime::new function with valid inputs
    #[test]
    fn test_datetime_new_valid() {
        let datetime = DateTime::new(2023, 1, 1, 0, 0, 0, TimeZone::UTC);
        assert!(datetime.is_ok());

        let datetime = DateTime::new(2024, 2, 29, 23, 59, 59, TimeZone::UTC); // Leap year
        assert!(datetime.is_ok());

        let datetime = DateTime::new(2023, 12, 31, 23, 59, 59, TimeZone::UTC);
        assert!(datetime.is_ok());
    }

    // Test the DateTime::new function with invalid inputs
    #[test]
    fn test_datetime_new_invalid() {
        // Invalid month
        let datetime = DateTime::new(2023, 0, 1, 0, 0, 0, TimeZone::UTC);
        assert!(datetime.is_err());

        let datetime = DateTime::new(2023, 13, 1, 0, 0, 0, TimeZone::UTC);
        assert!(datetime.is_err());

        // Invalid day
        let datetime = DateTime::new(2023, 2, 29, 0, 0, 0, TimeZone::UTC); // Not a leap year
        assert!(datetime.is_err());

        // Invalid hour
        let datetime = DateTime::new(2023, 1, 1, 24, 0, 0, TimeZone::UTC);
        assert!(datetime.is_err());

        // Invalid minute
        let datetime = DateTime::new(2023, 1, 1, 0, 60, 0, TimeZone::UTC);
        assert!(datetime.is_err());

        // Invalid second
        let datetime = DateTime::new(2023, 1, 1, 0, 0, 60, TimeZone::UTC);
        assert!(datetime.is_err());
    }

    // Test the now function for UTC timezone
    #[test]
    fn test_now_utc() {
        let current_time = now(TimeZone::UTC).unwrap();
        assert!(current_time.year >= 1970);
        assert!(current_time.month >= 1 && current_time.month <= 12);
        assert!(current_time.day >= 1 && current_time.day <= 31);
        assert!(current_time.hour <= 23);
        assert!(current_time.minute <= 59);
        assert!(current_time.second <= 59);
    }

    // Test the now function for KST timezone
    #[test]
    fn test_now_kst() {
        let current_time = now(TimeZone::KST).unwrap();
        assert!(current_time.year >= 1970);
        assert!(current_time.month >= 1 && current_time.month <= 12);
        assert!(current_time.day >= 1 && current_time.day <= 31);
        assert!(current_time.hour <= 23);
        assert!(current_time.minute <= 59);
        assert!(current_time.second <= 59);
    }

    // Test strftime formatting
    #[test]
    fn test_strftime() {
        let datetime = DateTime::new(2023, 8, 3, 5, 20, 59, TimeZone::UTC).unwrap();
        let formatted = datetime.strftime("%Y-%m-%d %H:%M:%S");
        assert_eq!(formatted, "2023-08-03 05:20:59");
    }

    // Test to_unix_seconds and from_unix_seconds functions
    #[test]
    fn test_unix_seconds_conversion() {
        let datetime = DateTime::new(2023, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        let unix_seconds = datetime.to_unix_seconds();
        let converted_datetime = DateTime::from_unix_seconds(unix_seconds, TimeZone::UTC).unwrap();
        assert_eq!(datetime, converted_datetime);

        // Test with different timezone
        let datetime_kst = DateTime::new(2023, 1, 1, 9, 0, 0, TimeZone::KST).unwrap();
        let unix_seconds_kst = datetime_kst.to_unix_seconds();
        let converted_datetime_kst =
            DateTime::from_unix_seconds(unix_seconds_kst, TimeZone::KST).unwrap();
        assert_eq!(datetime_kst, converted_datetime_kst);

        // Ensure unix_seconds are the same for UTC and KST adjusted for timezone
        assert_eq!(unix_seconds, unix_seconds_kst);
    }

    // Test add_timedelta function
    #[test]
    fn test_add_timedelta() {
        let datetime = DateTime::new(2023, 12, 31, 23, 59, 59, TimeZone::UTC).unwrap();
        let delta = TimeDelta {
            days: 1,
            seconds: 1,
            ..Default::default()
        };
        let new_datetime = datetime.add_timedelta(delta).unwrap();
        let expected_datetime = DateTime::new(2024, 1, 2, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime, expected_datetime);
    }

    // Test sub_timedelta function
    #[test]
    fn test_sub_timedelta() {
        let datetime = DateTime::new(2024, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        let delta = TimeDelta {
            days: 1,
            ..Default::default()
        };
        let new_datetime = datetime.sub_timedelta(delta).unwrap();
        let expected_datetime = DateTime::new(2023, 12, 31, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime, expected_datetime);
    }

    // Test addition operator overloading
    #[test]
    fn test_add_operator() {
        let datetime = DateTime::new(2023, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        let delta = TimeDelta {
            days: 30,
            ..Default::default()
        };
        let new_datetime = datetime + delta;
        let expected_datetime = DateTime::new(2023, 1, 31, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime.unwrap(), expected_datetime);
    }

    // Test subtraction operator overloading
    #[test]
    fn test_sub_operator() {
        let datetime = DateTime::new(2023, 1, 31, 0, 0, 0, TimeZone::UTC).unwrap();
        let delta = TimeDelta {
            days: 30,
            ..Default::default()
        };
        let new_datetime = datetime - delta;
        let expected_datetime = DateTime::new(2023, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime.unwrap(), expected_datetime);
    }

    // Test edge case transitions (e.g., leap years, month/year boundaries)
    #[test]
    fn test_edge_case_transitions() {
        // Leap year transition
        let datetime = DateTime::new(2024, 2, 28, 23, 59, 59, TimeZone::UTC).unwrap();
        let delta = TimeDelta {
            seconds: 1,
            ..Default::default()
        };
        let new_datetime = datetime.add_timedelta(delta).unwrap();
        let expected_datetime = DateTime::new(2024, 2, 29, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime, expected_datetime);

        // Non-leap year transition
        let datetime = DateTime::new(2023, 2, 28, 23, 59, 59, TimeZone::UTC).unwrap();
        let new_datetime = datetime.add_timedelta(delta).unwrap();
        let expected_datetime = DateTime::new(2023, 3, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime, expected_datetime);

        // Year boundary transition
        let datetime = DateTime::new(2023, 12, 31, 23, 59, 59, TimeZone::UTC).unwrap();
        let new_datetime = datetime.add_timedelta(delta).unwrap();
        let expected_datetime = DateTime::new(2024, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime, expected_datetime);
    }

    // Test timezone offset correctness
    #[test]
    fn test_timezone_offsets() {
        let datetime_utc = DateTime::new(2023, 8, 1, 12, 0, 0, TimeZone::UTC).unwrap();
        let datetime_kst = DateTime::new(2023, 8, 1, 21, 0, 0, TimeZone::KST).unwrap();

        // Both should represent the same point in time
        assert_eq!(
            datetime_utc.to_unix_seconds(),
            datetime_kst.to_unix_seconds()
        );

        let datetime_est = DateTime::new(2023, 8, 1, 7, 0, 0, TimeZone::EST).unwrap();
        assert_eq!(
            datetime_utc.to_unix_seconds(),
            datetime_est.to_unix_seconds()
        );
    }

    // Test TimeDelta display implementation
    #[test]
    fn test_timedelta_display() {
        let delta = TimeDelta {
            weeks: 1,
            days: 2,
            hours: 3,
            minutes: 4,
            seconds: 5,
        };
        let delta_str = format!("{}", delta);
        assert_eq!(delta_str, "1 week, 2 days, 3 hours, 4 minutes, 5 seconds");

        let delta = TimeDelta {
            weeks: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        };
        let delta_str = format!("{}", delta);
        assert_eq!(delta_str, "0 seconds");
    }

    // Test TimeDelta default implementation
    #[test]
    fn test_timedelta_default() {
        let delta = TimeDelta::default();
        assert_eq!(delta.weeks, 0);
        assert_eq!(delta.days, 0);
        assert_eq!(delta.hours, 0);
        assert_eq!(delta.minutes, 0);
        assert_eq!(delta.seconds, 0);
    }

    // Test error handling when subtracting more than current DateTime
    #[test]
    fn test_subtract_past_epoch() {
        let datetime = DateTime::new(1970, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        let delta = TimeDelta {
            days: 1,
            ..Default::default()
        };
        let result = datetime.sub_timedelta(delta);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Resulting DateTime is before Unix epoch (1970-01-01 00:00:00 UTC)"
        );
    }

    // Test adding negative TimeDelta
    #[test]
    fn test_add_negative_timedelta() {
        let datetime = DateTime::new(2023, 1, 2, 0, 0, 0, TimeZone::UTC).unwrap();
        let delta = TimeDelta {
            days: -1,
            ..Default::default()
        };
        let new_datetime = datetime.add_timedelta(delta).unwrap();
        let expected_datetime = DateTime::new(2023, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(new_datetime, expected_datetime);
    }

    // Test TimeZone offset_in_seconds correctness
    #[test]
    fn test_timezone_offset_in_seconds() {
        assert_eq!(TimeZone::UTC.offset_in_seconds(), 0);
        assert_eq!(TimeZone::KST.offset_in_seconds(), 9 * 3600);
        assert_eq!(TimeZone::EST.offset_in_seconds(), -5 * 3600);
        assert_eq!(TimeZone::IST.offset_in_seconds(), 5 * 3600 + 1800); // 5 hours 30 minutes
    }

    // Test compute_total_seconds utility function
    #[test]
    fn test_compute_total_seconds() {
        let total_seconds = compute_total_seconds(1, 1, 1, 1, 1);
        let expected_seconds = SECONDS_IN_WEEK as i64
            + SECONDS_IN_DAY as i64
            + SECONDS_IN_HOUR as i64
            + SECONDS_IN_MINUTE as i64
            + 1;
        assert_eq!(total_seconds, expected_seconds);
    }

    // Test adjust_second_with_timezone utility function
    #[test]
    fn test_adjust_second_with_timezone() {
        let total_seconds = 1000;
        let adjusted_seconds = adjust_second_with_timezone(total_seconds, TimeZone::KST);
        assert_eq!(adjusted_seconds, total_seconds + (9 * 3600) as u64);
    }

    // Test calculate_date_since_epoch utility function
    #[test]
    fn test_calculate_date_since_epoch() {
        let adjusted_seconds = 0;
        let datetime = calculate_date_since_epoch(adjusted_seconds, TimeZone::UTC).unwrap();
        let expected_datetime = DateTime::new(1970, 1, 1, 0, 0, 0, TimeZone::UTC).unwrap();
        assert_eq!(datetime, expected_datetime);
    }

    #[test]
    fn test_timezone_offsets_correctness() {
        assert_eq!(TimeZone::UTC.offset_in_seconds(), 0);
        assert_eq!(TimeZone::KST.offset_in_seconds(), 32400);
        assert_eq!(TimeZone::EST.offset_in_seconds(), -18000);
        assert_eq!(TimeZone::PST.offset_in_seconds(), -28800);
        assert_eq!(TimeZone::IST.offset_in_seconds(), 19800);
    }
    #[test]
    fn test_from_unix_seconds_debug() {
        let unix_seconds = 1690891200; // UTC 기준 2023-08-01 12:00:00
        let datetime_utc = DateTime::from_unix_seconds(unix_seconds, TimeZone::UTC).unwrap();
        let datetime_kst = DateTime::from_unix_seconds(unix_seconds, TimeZone::KST).unwrap();

        println!("UTC DateTime: {:?}", datetime_utc);
        println!("KST DateTime: {:?}", datetime_kst);

        assert_eq!(datetime_utc.to_unix_seconds(), unix_seconds);
        assert_eq!(datetime_kst.to_unix_seconds(), unix_seconds);
    }

    #[test]
    fn test_all_timezone_offsets() {
        let unix_seconds = 1690891200; // UTC 기준 2023-08-01 12:00:00

        let timezones = vec![
            TimeZone::UTC,
            TimeZone::KST,
            TimeZone::EST,
            TimeZone::PST,
            TimeZone::JST,
            TimeZone::IST,
            TimeZone::CET,
            TimeZone::AST,
            TimeZone::CST,
            TimeZone::MST,
            TimeZone::AKST,
            TimeZone::HST,
            TimeZone::BST,
            TimeZone::WET,
            TimeZone::EET,
            TimeZone::SAST,
            TimeZone::EAT,
            TimeZone::AEST,
            TimeZone::ACST,
            TimeZone::AWST,
            TimeZone::CSTAsia,
            TimeZone::SGT,
            TimeZone::HKT,
        ];

        for timezone in &timezones {
            let datetime = DateTime::from_unix_seconds(unix_seconds, *timezone).unwrap();
            let recalculated_unix = datetime.to_unix_seconds();

            println!(
                "Timezone: {:?}, DateTime: {:?}, Recalculated Unix: {}, Expected Unix: {}",
                timezone, datetime, recalculated_unix, unix_seconds
            );

            assert_eq!(
                recalculated_unix, unix_seconds,
                "Timezone {:?} failed",
                timezone
            );
        }
    }

    #[test]
    fn test_datetime_new_correctness_debug() {
        let datetime_utc = DateTime::new(2023, 8, 1, 12, 0, 0, TimeZone::UTC).unwrap();
        let datetime_kst = DateTime::new(2023, 8, 1, 21, 0, 0, TimeZone::KST).unwrap();
        let datetime_est = DateTime::new(2023, 8, 1, 7, 0, 0, TimeZone::EST).unwrap();

        println!("UTC: {:?}", datetime_utc);
        println!("KST: {:?}", datetime_kst);
        println!("EST: {:?}", datetime_est);

        println!(
            "UTC to Unix Seconds: {}, KST to Unix Seconds: {}, EST to Unix Seconds: {}",
            datetime_utc.to_unix_seconds(),
            datetime_kst.to_unix_seconds(),
            datetime_est.to_unix_seconds()
        );

        assert_eq!(datetime_utc.to_unix_seconds(), 1690891200);
        assert_eq!(datetime_kst.to_unix_seconds(), 1690891200);
        assert_eq!(datetime_est.to_unix_seconds(), 1690891200);
    }
}
