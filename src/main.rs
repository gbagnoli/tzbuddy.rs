use anyhow::Result;
use clap::Parser;
use tzbuddy::{calculate_timezone_hours, get_timezones, get_utc_date, print_table, Cli, SortOrder};

fn main() -> Result<()> {
    let cli = Cli::parse();
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
