
## [0.2.0] - 2024-12-1

### Added
- Introduced `TimeDelta` struct for performing time arithmetic (add/subtract days, hours, minutes, and seconds).
- Extended `DateTime` functionality to support addition and subtraction of `TimeDelta`.
- Updated README to include usage examples for time arithmetic.

## [0.1.4] - 2024-11-14

### Changed
- is_learp_year function changed fn -> const fn


## [0.1.2] - 2024-11-14

### Added
- time condition checking when create DateTime
- it returns Result<DateTime, String>
- constant values SECONDS_IN_YEAR, SECONDS_IN_MONTH, SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE