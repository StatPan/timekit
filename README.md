
# TimeKit

`TimeKit` is a simple and lightweight Rust library for working with timezones and displaying the current date and time in multiple time zones. This library is designed to be easy to use and efficient, providing hardcoded timezone offsets to avoid runtime computation and enhance performance.

## Project Purpose

The purpose of `TimeKit` is to offer a flexible and convenient way to work with different time zones in Rust, allowing developers to retrieve and display the current time in a variety of world time zones. This project was built to address the need for a lightweight timezone-handling library without external dependencies, focusing on simplicity and usability.

In the globalized world we live in, applications often need to support users in different regions. Having a reliable and efficient method to calculate time across multiple time zones is essential for many systems, including scheduling applications, communication tools, and data logging systems.

## Why TimeKit?

- **Lightweight**: No external dependencies, just pure Rust code.
- **Efficient**: Timezone offsets are hardcoded to avoid any additional computation during runtime.
- **Simple API**: Easy-to-use methods to get the current date and time for any supported time zone.
- **Comprehensive**: Supports a wide range of time zones, including all major global regions.
- **Time Arithmetic**: Easily add or subtract time intervals with `TimeDelta`.

## Features

- Retrieve the current date and time in multiple time zones.
- Perform time arithmetic with `TimeDelta` (e.g., add or subtract days, hours, minutes, or seconds).
- Hardcoded timezone offsets for better performance.
- Formats output in a human-readable `YYYY-MM-DD HH:MM:SS` format.
- Easy to use API with minimal setup.

## Installation

To start using `TimeKit`, add the following line to your `Cargo.toml` file under `[dependencies]`:

```toml
[dependencies]
timekit = "0.2.0"
```

Then, in your Rust code:

```rust
use timekit;
```

## Usage

### Retrieving Current Time
Here's a simple example showing how to use TimeKit to get the current time in various time zones.

```rust
use timekit::TimeZone;
use timekit::now;

fn main() {
    // Get the current time in UTC
    let utc_time = now(TimeZone::UTC).unwrap();
    println!("Current UTC time: {}", utc_time);

    // Get the current time in Korea Standard Time (KST)
    let kst_time = now(TimeZone::KST).unwrap();
    println!("Current KST time: {}", kst_time);

    // Get the current time in Eastern Standard Time (EST)
    let est_time = now(TimeZone::EST).unwrap();
    println!("Current EST time: {}", est_time);
}
```

### Performing Time Arithmetic

You can perform time arithmetic such as adding or subtracting time using the `TimeDelta` struct.

```rust
use timekit::{DateTime, TimeDelta, TimeZone};

fn main() {
    let datetime = DateTime::new(2023, 8, 1, 12, 0, 0, TimeZone::UTC).unwrap();

    // Add 1 day and 3 hours to the datetime
    let new_datetime = datetime + TimeDelta {
        days: 1,
        hours: 3,
        ..Default::default()
    };
    println!("New DateTime after addition: {}", new_datetime.unwrap());

    // Subtract 2 days and 6 hours from the datetime
    let earlier_datetime = datetime - TimeDelta {
        days: 2,
        hours: 6,
        ..Default::default()
    };
    println!("New DateTime after subtraction: {}", earlier_datetime.unwrap());
}
```

## Supported Time Zones

TimeKit supports a wide variety of time zones. Here are some of the supported zones:

- UTC: Coordinated Universal Time (UTC+0)  
- KST: Korea Standard Time (UTC+9)  
- EST: Eastern Standard Time (UTC-5)  
- PST: Pacific Standard Time (UTC-8)  
- JST: Japan Standard Time (UTC+9)  
- IST: India Standard Time (UTC+5:30)  
- CET: Central European Time (UTC+1)  
- AST: Atlantic Standard Time (UTC-4)  
- CST: Central Standard Time (UTC-6)  
- MST: Mountain Standard Time (UTC-7)  
- AKST: Alaska Standard Time (UTC-9)  
- HST: Hawaii Standard Time (UTC-10)  
- BST: British Summer Time (UTC+1)  
- WET: Western European Time (UTC+0)  
- EET: Eastern European Time (UTC+2)  
- SAST: South Africa Standard Time (UTC+2)  
- EAT: East Africa Time (UTC+3)  
- AEST: Australian Eastern Standard Time (UTC+10)  
- ACST: Australian Central Standard Time (UTC+9:30)  
- AWST: Australian Western Standard Time (UTC+8)  
- CST (Asia): China Standard Time (UTC+8)  
- SGT: Singapore Time (UTC+8)  
- HKT: Hong Kong Time (UTC+8)  

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, feel free to submit a pull request or create an issue in the repository.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
