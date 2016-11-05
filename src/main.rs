extern crate clap;
extern crate chrono;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::*;

use clap::{Arg, App};
use chrono::*;

pub const DATE_FORMAT: &'static str = "%Y-%m-%d";

fn main() {

    let matches = App::new("Quant Software")
        .version("0.1")
        .author("Marek Dudek")
        .arg(Arg::with_name("start-date").long("start-date").takes_value(true))
        .arg(Arg::with_name("end-date").long("end-date").takes_value(true))
        .get_matches();

    let start_date = extract_date_with_default(matches.value_of("start-date"), NaiveDate::from_ymd(1962, 7, 5));
    let end_date = extract_date_with_default(matches.value_of("end-date"), NaiveDate::from_ymd(2131, 1, 2));
    println!("Processing for period between {} and {}.", start_date, end_date);
    fetch_data();
}

pub fn extract_date_with_default(param: Option<&str>, default: NaiveDate) -> NaiveDate {
    let result: Option<Result<NaiveDate, ParseError>> =
        param.map(|s| NaiveDate::parse_from_str(s, DATE_FORMAT));
    result.map(|r| r.unwrap()).unwrap_or(default)
}

pub fn fetch_data() {
    println!("Fetchin data");
    let path = "/usr/local/lib/python2.7/dist-packages/QSTK-0.2.8-py2.7.egg/QSTK/QSData/Yahoo/GOOG.csv";
    let file = File::open(path).expect("no such file");
    let buffer = BufReader::new(file);
    let lines:Vec<String> = buffer.lines().map(|l| l.expect("could not parse")).collect();
    println!("Length of file: {}", lines.len());
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

    #[test]
    fn fetching_data() {
        fetch_data();
    }
}
