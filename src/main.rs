extern crate clap;
extern crate chrono;

use clap::{Arg, App};
use chrono::*;

pub const DATE_FORMAT: &'static str = "%Y-%m-%d";

fn main() {

    let nyse_start_date = NaiveDate::from_ymd(1962, 7, 5);
    let nyse_end_date = NaiveDate::from_ymd(2131, 1, 2);
    let nyse_cob = NaiveTime::from_hms(16, 0, 0);

    let matches = App::new("Quant Software")
        .version("0.1")
        .author("Marek Dudek")
        .arg(Arg::with_name("start-date").long("start-date").takes_value(true))
        .arg(Arg::with_name("end-date").long("end-date").takes_value(true))
        .get_matches();

    let start_date = extract_date_with_default(matches.value_of("start-date"), nyse_start_date);
    let end_date = extract_date_with_default(matches.value_of("end-date"), nyse_end_date);
    println!("Processing for period between {} and {} (at {}).", start_date, end_date, nyse_cob);
}

pub fn extract_date_with_default(param: Option<&str>, default: NaiveDate) -> NaiveDate {
    let result: Option<Result<NaiveDate, ParseError>> =
        param.map(|s| NaiveDate::parse_from_str(s, DATE_FORMAT));
    result.map(|r| r.unwrap()).unwrap_or(default)
}


#[cfg(test)]
mod tests {

    use super::*;
    use chrono::*;

    #[test]
    fn parsing_date_with_default() {
        let date = extract_date_with_default(Some("1962-07-05"), NaiveDate::from_ymd(1962, 7, 5));
        assert_eq!(date, NaiveDate::from_ymd(1962, 7, 5));
    }
}
