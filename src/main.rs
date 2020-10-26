extern crate clap;
extern crate chrono;
extern crate chrono_tz;

use std::collections::HashMap;
use chrono_tz::Tz;
use clap::{App, Values};
use chrono::{DateTime, Utc, Duration, Timelike};

fn get_timezones(cmdline_tzs: Option<Values<'_>>) -> HashMap<&str, Tz> {
    let mut timezones = HashMap::new();
    match cmdline_tzs {
        Some(values) => {
            for tz_str in values {
                let tz_opt = tz_str.parse();
                match tz_opt {
                    Ok(tz) => {
                        timezones.insert(tz_str, tz);
                    }
                    Err(e) => println!("{}: discarding", e),
                }
            }
        }
        None => println!("No timezones provided."),

    }
    timezones
}

fn get_utc_date(cmdline_date: Option<&str>) -> DateTime<Utc> {
    match cmdline_date {
        Some(date) => {
            let format = "%Y-%m-%d %H:%M";
            match chrono::NaiveDateTime::parse_from_str(date, format) {
                Ok(naive) => chrono::DateTime::from_utc(naive, chrono::Utc),
                Err(e) => {
                    println!(
                        "Invalid date '{}' for format '{}' ({}). Using now()",
                        date,
                        format,
                        e
                    );
                    chrono::Utc::now()
                }
            }
        }
        None => chrono::Utc::now(),
    }
}

fn get_hours_matrix(
    tzs: HashMap<&str, Tz>,
    date: DateTime<Utc>,
    span: i32,
) -> HashMap<&str, Vec<u32>> {
    let half_span = span - span / 2;
    let mut matrix = HashMap::new();
    for (tz_str, tz) in &tzs {
        let mut hours = Vec::new();
        for i in 0..span {
            let offset = i64::from(i - half_span);
            let converted = date.with_timezone(tz);
            let mut shifted = converted
                .checked_add_signed(Duration::hours(offset))
                .unwrap();
            if offset < 0 {
                shifted = converted
                    .checked_sub_signed(Duration::hours(-offset))
                    .unwrap();
            }
            hours.push(shifted.hour())
        }
        matrix.insert(*tz_str, hours);
    }
    matrix
}

fn get_span(cmdline_span: Option<&str>) -> i32 {
    let default = 12;
    match cmdline_span {
        Some(span) => {
            match span.parse::<i32>() {
                Ok(val) => val,
                Err(e) => {
                    println!(
                        "Cannot parse {} as int: {}. Using default {}",
                        span,
                        e,
                        default
                    );
                    default
                }
            }
        }
        None => default,
    }
}

fn main() {
    let config_yaml = clap::load_yaml!("args.yaml");
    let matches = App::from(config_yaml).get_matches();
    let matrix = get_hours_matrix(
        get_timezones(matches.values_of("timezones")),
        get_utc_date(matches.value_of("date")),
        get_span(matches.value_of("span")),
    );
    println!("Hello, world!: {:?}", matrix);
}
