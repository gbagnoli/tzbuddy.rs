use anyhow::Result;
use chrono_tz::Tz;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tzbuddy::{
    SortOrder, calculate_timezone_hours, generate_table, get_region_timezones, get_utc_date,
};

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
    /// list timezones for a given continent.
    /// If no continent is set, then prints continents
    #[clap(short = 'L', long = "list-timezones")]
    list_timezones: Option<Option<String>>,
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Some(arg) = cli.list_timezones {
        for tz in get_region_timezones(arg)? {
            println!("{tz}");
        }
        return Ok(());
    }
    if cli.save {
        println!(
            "Saving config at {}",
            confy::get_configuration_file_path("tzbuddy", "tzbuddy")?.display()
        );
        confy::store("tzbuddy", "tzbuddy", &cli)?;
    }
    let config: Cli = if cli.noconfig {
        Cli::default()
    } else {
        confy::load("tzbuddy", "tzbuddy")?
    };

    let date = get_utc_date(cli.date)?;
    let mut sort_order = SortOrder::East;
    if cli.noorder || config.noorder {
        sort_order = SortOrder::None;
    } else if cli.inverseorder || config.inverseorder {
        sort_order = SortOrder::West;
    }
    let mut timezones: Vec<(String, Tz)> = match cli.timezones.len() {
        0 => config
            .timezones
            .into_iter()
            .map(|tz| (tz.clone(), tz.parse().unwrap()))
            .collect(),
        _ => cli
            .timezones
            .into_iter()
            .map(|tz| (tz.clone(), tz.parse().unwrap()))
            .collect(),
    };
    if timezones.is_empty() {
        println!("No timezones provided.");
    }
    // Dedup timezones, but preserve the order
    let mut seen = HashSet::new();
    timezones.retain(|x| seen.insert(x.0.clone()));

    let span = match cli.span {
        None => config.span.unwrap_or(12),
        Some(span) => span,
    };
    let ampm = cli.ampm || config.ampm;
    let tzhours = calculate_timezone_hours(timezones, date, span, ampm, sort_order);
    generate_table(tzhours, date, cli.noheader || config.noheader, ampm).printstd();
    Ok(())
}
