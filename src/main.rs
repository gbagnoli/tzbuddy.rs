extern crate clap;
extern crate chrono;
extern crate chrono_tz;

use std::collections::HashMap;
use chrono_tz::Tz;
use clap::{App, Values};
use chrono::{DateTime, Utc};

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

fn main() {
    let config_yaml = clap::load_yaml!("args.yaml");
    let matches = App::from(config_yaml).get_matches();
    let _timezones = get_timezones(matches.values_of("timezones"));
    let date = get_utc_date(matches.value_of("date"));
    println!("Using date: {}", date);
    println!("Hello, world!");
}
