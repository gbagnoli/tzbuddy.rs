use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Timelike, Utc};
use chrono_tz::Tz;
use clap::Parser;
use prettytable::{format, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq)]
struct TimezoneHours {
    name: String,
    hours: Vec<String>,
    tz: Tz,
}

#[derive(Parser, Debug, Serialize, Deserialize, Default)]
#[clap(name = clap::crate_name!())]
#[clap(author = clap::crate_authors!("\n"))]
#[clap(version=clap::crate_version!())]
#[clap(about=clap::crate_description!())]
struct Cli {
    /// Which timezone(s) to display.
    /// List at: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
    #[clap(short = 'z', long = "tz")]
    timezones: Vec<String>,
    /// Do not order timezones
    #[clap(short = 'O', long = "no-order", group = "order")]
    noorder: bool,
    /// Sort TZ west to east
    #[clap(short = 'I', long = "inverse-order", group = "order")]
    inverseorder: bool,
    /// Do not display header
    #[clap(short = 'H', long = "no-header")]
    noheader: bool,
    /// How many hours to span
    #[clap(short, long)]
    span: Option<i32>,
    /// Use 12h (am/pm) format
    #[clap(short, long = "am-pm")]
    ampm: bool,
    /// Save config with current args
    #[clap(long)]
    save: bool,
    /// Do not load config at startup
    #[clap(long = "no-config")]
    noconfig: bool,
    ///Calculate times from a specific date (YYYY-mm-dd HH:mm). If omitted, current time is used
    date: Option<String>,
}

impl TimezoneHours {
    fn naive_local_timestamp(&self, date: DateTime<Utc>) -> i64 {
        date.with_timezone(&self.tz).naive_local().timestamp()
    }
}
// ordering here might seem "backwards" but we want eastern TZs to come first i.e. they are
// threated as "lesser so that calling vec.sort() would have sorting starting from TimezoneHours
// strucs with eastern timezone - which has bigger value of naive_local_timestamp(now);
// to do that, all comparisons are other.cmp(self) not the other way around
impl Ord for TimezoneHours {
    fn cmp(&self, other: &Self) -> Ordering {
        let now = Utc::now();
        other
            .naive_local_timestamp(now)
            .cmp(&self.naive_local_timestamp(now))
    }
}

impl PartialOrd for TimezoneHours {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimezoneHours {
    fn eq(&self, other: &Self) -> bool {
        let now = Utc::now();
        self.naive_local_timestamp(now) == other.naive_local_timestamp(now)
    }
}

enum SortOrder {
    None,
    East,
    West,
}

fn get_timezones(cmdline_tzs: Vec<String>) -> HashMap<String, Tz> {
    let mut timezones = HashMap::new();
    if !cmdline_tzs.is_empty() {
        for tz_str in cmdline_tzs {
            let tz_opt = tz_str.parse();
            match tz_opt {
                Ok(tz) => {
                    timezones.insert(tz_str, tz);
                }
                Err(e) => println!("{}: discarding", e),
            }
        }
    } else {
        println!("No timezones provided.");
    }
    timezones
}

fn get_utc_date(cmdline_date: Option<String>) -> DateTime<Utc> {
    match cmdline_date {
        Some(date) => {
            let format = "%Y-%m-%d %H:%M";
            match NaiveDateTime::parse_from_str(&date, format) {
                Ok(naive) => DateTime::from_utc(naive, Utc),
                Err(e) => {
                    println!(
                        "Invalid date '{}' for format '{}' ({}). Using now()",
                        date, format, e
                    );
                    Utc::now()
                }
            }
        }
        None => Utc::now(),
    }
}

fn calculate_timezone_hours(
    tzs: HashMap<String, Tz>,
    date: DateTime<Utc>,
    span: i32,
    am_pm: bool,
    sort_order: SortOrder,
) -> Vec<TimezoneHours> {
    let half_span = (span / 2) - 1;
    let mut tzhours = Vec::new();
    for (tz_str, tz) in tzs {
        let mut hours = Vec::new();
        for i in 0..span {
            let offset = i64::from(i - half_span);
            let utc_day = date.day();
            let converted = date.with_timezone(&tz);
            let mut shifted = converted
                .checked_add_signed(Duration::hours(offset))
                .unwrap();
            if offset < 0 {
                shifted = converted
                    .checked_sub_signed(Duration::hours(-offset))
                    .unwrap();
            }
            let sfx = match shifted.day().cmp(&utc_day) {
                Ordering::Greater => "+",
                Ordering::Less => "-",
                Ordering::Equal => " ",
            };
            let mut hour = format!("{:>02}", shifted.hour());
            if am_pm {
                let (pm, h) = shifted.hour12();
                if pm {
                    hour = format!("{:>2} pm", h);
                } else {
                    hour = format!("{:>2} am", h);
                }
            }
            if offset == 0 {
                // current hour!
                hours.push(format!("| {:>02}{}|", hour, sfx));
            } else {
                hours.push(format!(" {:>02}{}", hour, sfx));
            }
        }
        tzhours.push(TimezoneHours {
            name: tz_str,
            tz,
            hours,
        });
    }
    match sort_order {
        SortOrder::None => tzhours,
        SortOrder::East => {
            tzhours.sort();
            tzhours
        }
        SortOrder::West => {
            tzhours.sort_by(|a, b| b.cmp(a));
            tzhours
        }
    }
}

fn print_table(tz_hours: Vec<TimezoneHours>, date: DateTime<Utc>, no_header: bool, am_pm: bool) {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(&[], format::LineSeparator::new('-', '+', '+', '+'))
        .build();

    table.set_format(format);
    let tz_format = if am_pm {
        "(%Z) %a %l:%M %P %d/%m/%Y"
    } else {
        "(%Z) %a %H:%M %d/%m/%Y"
    };
    for hours in tz_hours {
        let converted = date.with_timezone(&hours.tz);
        let mut row_elems = Vec::new();
        if !no_header {
            row_elems.push(Cell::new(&hours.name));
            row_elems.push(Cell::new(&converted.format(tz_format).to_string()));
            row_elems.push(Cell::new("·"));
        }
        for hour in hours.hours {
            row_elems.push(Cell::new(&hour));
        }
        table.add_row(Row::new(row_elems));
    }
    table.printstd();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.save {
        println!("Saving config");
        confy::store("tzbuddy", &cli)?;
    }
    let config: Cli = if cli.noconfig {
        Cli::default()
    } else {
        confy::load("tzbuddy")?
    };

    let date = get_utc_date(cli.date);
    let mut sort_order = SortOrder::East;
    if cli.noorder || config.noorder {
        sort_order = SortOrder::None;
    } else if cli.inverseorder || config.inverseorder {
        sort_order = SortOrder::West;
    }
    let timezones = match cli.timezones.len() {
        0 => config.timezones,
        _ => cli.timezones,
    };
    let span = match cli.span {
        None => config.span.unwrap_or(12),
        Some(span) => span,
    };
    let ampm = cli.ampm || config.ampm;
    let tzhours = calculate_timezone_hours(get_timezones(timezones), date, span, ampm, sort_order);
    print_table(tzhours, date, cli.noheader || config.noheader, ampm);
    Ok(())
}
