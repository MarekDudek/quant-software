extern crate clap;
extern crate chrono;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::*;
use std::str::*;

use clap::{Arg, App};
use chrono::*;

pub const DATE_FORMAT: &'static str = "%Y-%m-%d";

#[allow(unused_variables)]
fn main() {

    let all_tickers: Vec<String> = read_all_tickers();

    let matches = App::new("Quant Software")
        .version("0.1")
        .author("Marek Dudek")
        .arg(Arg::with_name("start-date").long("start-date").takes_value(true))
        .arg(Arg::with_name("end-date").long("end-date").takes_value(true))
        .arg(Arg::with_name("tickers").long("tickers").takes_value(true))
        .get_matches();

    let start_date = extract_date_with_default(matches.value_of("start-date"),
                                               NaiveDate::from_ymd(1962, 7, 5));
    let end_date = extract_date_with_default(matches.value_of("end-date"),
                                             NaiveDate::from_ymd(2131, 1, 2));
    let tickers = extract_tickers_with_default(matches.value_of("tickers"), all_tickers);

    println!("Processing for period between {} and {}.",
             start_date,
             end_date);
    println!("Tickers: {}", tickers.join(", "));

    // fetch_data(&all_tickers);
}

pub fn extract_date_with_default(param: Option<&str>, default: NaiveDate) -> NaiveDate {
    let result: Option<Result<NaiveDate, ParseError>> =
        param.map(|s| NaiveDate::parse_from_str(s, DATE_FORMAT));
    result.map(|r| r.unwrap()).unwrap_or(default)
}

pub fn read_all_tickers() -> Vec<String> {
    let path = "./src/resources/tickers.txt";
    let file = File::open(path).expect("No tickers file");
    let buffer = BufReader::new(file);
    let lines = buffer.lines().map(|l| l.expect("cound not parse ticker line"));
    lines.collect()
}

pub fn extract_tickers_with_default<'a>(param: Option<&'a str>,
                                        default: Vec<String>)
                                        -> Vec<String> {
    let tokens: Option<Split<&str>> = param.map(|s| s.split(","));
    let vector: Option<Vec<&str>> = tokens.map(|ts| ts.collect::<Vec<&str>>());
    let strings: Option<Vec<String>> = vector.map(|v| v.iter().map(|s| s.to_string()).collect());
    strings.unwrap_or(default)
}

#[allow(unused_variables)]
pub fn fetch_data(tickers: &Vec<&str>) {
    println!("Fetchin data");
    let path = "/usr/local/lib/python2.7/dist-packages/QSTK-0.2.8-py2.7.\
                egg/QSTK/QSData/Yahoo/GOOG.csv";
    let file = File::open(path).expect("no such file");
    let buffer = BufReader::new(file);
    let lines: Vec<String> = buffer.lines().map(|l| l.expect("could not parse")).collect();
    println!("Length of file: {}", lines.len());
}


#[cfg(test)]
mod tests {

    use super::*;
    use chrono::*;

    #[test]
    fn parsing_date_with_default() {
        let expected = NaiveDate::from_ymd(1962, 7, 5);
        let actual = extract_date_with_default(Some("1962-07-05"), expected);
        assert_eq!(expected, actual);
    }

    #[test]
    fn extracting_tickets() {
        let param = "A,B,C";
        let expected = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let actual = extract_tickers_with_default(Some(param), vec![]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn fetching_data() {
        fetch_data(&vec!["AA"]);
    }
}
