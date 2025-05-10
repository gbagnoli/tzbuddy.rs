use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tzbuddy::{
    SortOrder, calculate_timezone_hours, get_timezones, get_utc_date, print_table, print_timezones,
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
        return print_timezones(arg);
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
