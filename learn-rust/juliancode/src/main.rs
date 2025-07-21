use time::{OffsetDateTime, UtcOffset};

fn main() {
    let offset = UtcOffset::from_hms(-4, 0, 0).unwrap();
    let now = OffsetDateTime::now_utc().to_offset(offset);
    let julian_barcode = get_julian(&now);
    println!("{}", julian_barcode);
}

fn get_julian(curr: &OffsetDateTime) -> String{
    let year = curr.year() % 100;
    let julian = curr.date().ordinal();

    let total_daily_secs = 24 * 60 * 60;
    let elapsed_secs = (curr.hour() as u32) * 3600 + (curr.minute() as u32) * 60 + (curr.second() as u32);
    let time_stamp = 1 + ((elapsed_secs as f64 / total_daily_secs as f64) * 19999.0).round() as u32;

    format!("{:02}-{:03}-{:05}", year, julian, time_stamp)
}
