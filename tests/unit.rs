use chrono::Datelike;
use chrono::Timelike;
use tzbuddy::REGIONS;
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
