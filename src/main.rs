extern crate clap;

fn get_timezones(cmdline_tzs: Option<clap::Values<'_>>) -> Vec<&str> {
    let mut timezones = Vec::new();
    match cmdline_tzs {
        Some(values) => {
            for tz in values {
                timezones.push(tz);
            }
        }
        None => {
            println!("todo implement local time");
        }

    }
    timezones
}

fn main() {
    let config_yaml = clap::load_yaml!("args.yaml");
    let matches = clap::App::from(config_yaml).get_matches();

    let _timezones = get_timezones(matches.values_of("timezones"));
    println!("Hello, world!");
}
