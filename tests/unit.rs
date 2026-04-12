use chrono::Datelike;
use chrono::Timelike;
use tzbuddy::REGIONS;
use tzbuddy::get_timezones;
use tzbuddy::get_utc_date;

#[test]
fn test_regions_loaded() {
    assert!(!REGIONS.is_empty());
    assert!(REGIONS.contains_key("Europe"));
}

#[test]
fn test_get_utc_date() {
    let date = get_utc_date(Some("2023-10-27 10:00".to_string())).unwrap();
    assert_eq!(date.year(), 2023);
    assert_eq!(date.month(), 10);
    assert_eq!(date.day(), 27);
    assert_eq!(date.hour(), 10);
    assert_eq!(date.minute(), 0);
}

#[test]
fn test_get_timezones() {
    let tzs = vec!["Europe/Rome".to_string(), "UTC".to_string()];
    let res = get_timezones(tzs).unwrap();
    assert_eq!(res.len(), 2);
    assert!(res.contains_key("Europe/Rome"));
    assert!(res.contains_key("UTC"));
}

#[test]
fn test_invalid_timezone() {
    let tzs = vec!["Invalid/TZ".to_string()];
    let res = get_timezones(tzs);
    assert!(res.is_err());
}
