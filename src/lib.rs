use anyhow::{Result, bail};
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Timelike, Utc};
use chrono_tz::{TZ_VARIANTS, Tz};
use prettytable::{Cell, Row, Table, format};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn by_regions() -> BTreeMap<&'static str, BTreeSet<Option<&'static str>>> {
    let mut res: BTreeMap<&'static str, BTreeSet<Option<&'static str>>> = BTreeMap::new();
    for tz in TZ_VARIANTS.iter() {
        let name = tz.name();
        let mut parts = name.split('/');
        let region = parts.next().unwrap_or("Unknown");
        let location = parts.next();
        match res.get_mut(&region) {
            Some(tzs) => {
                tzs.insert(location);
            }
            None => {
                let mut tzs = BTreeSet::new();
                tzs.insert(location);
                res.insert(region, tzs);
            }
        }
    }
    res
}

fn print_regions() -> Result<()> {
    for (region, _tzs) in by_regions().iter() {
        println!("{region}");
    }
    Ok(())
}

pub fn print_timezones(arg: Option<String>) -> Result<()> {
    match arg {
        Some(region) => {
            match by_regions().get(&region.as_str()) {
                Some(tzs) => {
                    for tz in tzs.iter() {
                        match tz {
                            Some(x) => {
                                println!("{region}/{x}");
                            }
                            None => {
                                println!("{region}");
                            }
                        }
                    }
                }
                None => {
                    bail!("Cannot find region {}", region);
                }
            }
            Ok(())
        }
        None => print_regions(),
    }
}

#[derive(Debug, Eq)]
pub struct TimezoneHours {
    name: String,
    hours: Vec<String>,
    tz: Tz,
}

impl TimezoneHours {
    fn naive_local_timestamp(&self, date: DateTime<Utc>) -> i64 {
        date.with_timezone(&self.tz)
            .naive_local()
            .and_utc()
            .timestamp()
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

pub enum SortOrder {
    None,
    East,
    West,
}

pub fn get_timezones(cmdline_tzs: Vec<String>) -> HashMap<String, Tz> {
    let mut timezones = HashMap::new();
    if !cmdline_tzs.is_empty() {
        for tz_str in cmdline_tzs {
            let tz_opt = tz_str.parse();
            match tz_opt {
                Ok(tz) => {
                    timezones.insert(tz_str, tz);
                }
                Err(e) => println!("{e}: discarding"),
            }
        }
    } else {
        println!("No timezones provided.");
    }
    timezones
}

pub fn get_utc_date(cmdline_date: Option<String>) -> DateTime<Utc> {
    match cmdline_date {
        Some(date) => {
            let format = "%Y-%m-%d %H:%M";
            match NaiveDateTime::parse_from_str(&date, format) {
                Ok(naive) => DateTime::from_naive_utc_and_offset(naive, Utc),
                Err(e) => {
                    println!("Invalid date '{date}' for format '{format}' ({e}). Using now()");
                    Utc::now()
                }
            }
        }
        None => Utc::now(),
    }
}

pub fn calculate_timezone_hours(
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
                    hour = format!("{h:>2} pm");
                } else {
                    hour = format!("{h:>2} am");
                }
            }
            if offset == 0 {
                // current hour!
                hours.push(format!("| {hour:>02}{sfx}|"));
            } else {
                hours.push(format!(" {hour:>02}{sfx}"));
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

pub fn print_table(
    tz_hours: Vec<TimezoneHours>,
    date: DateTime<Utc>,
    no_header: bool,
    am_pm: bool,
) {
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
