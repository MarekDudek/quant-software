extern crate clap;
extern crate chrono;

use clap::{Arg, App};
use chrono::*;

pub const STRING: &'static str = "static string";

fn main() {

    let NYSE_START_DATE = NaiveDate::from_ymd(1962, 7, 5);
    let NYSE_END_DATE = NaiveDate::from_ymd(2131, 1, 2);
    let DATE_FORMAT = "%Y-%m-%d";

    let matches = App::new("Quant Software ")
        .version("0.1")
        .author("Marek Dudek")
        .arg(Arg::with_name("from").long("from").takes_value(true))
        .get_matches();

    let f1:Option<&str> = matches.value_of("from");//.map(|o| NaiveDate::parse_from_str(s, DATE_FORMAT));//.unwrap_or(NYSE_START_DATE);
    let f2:Option<Result<_, _>> = f1.map(|s| NaiveDate::parse_from_str(s, DATE_FORMAT));
    let f3:Option<NaiveDate> = f2.map(|r| r.unwrap());
    let f4: NaiveDate = f2.map(|r| r.unwrap()).unwrap_or(NYSE_START_DATE);
    println!("from: {:?}", f1);
    println!("from: {:?}", f2);
    println!("from: {:?}", f3);
    println!("from: {:?}", f4);
}


#[cfg(test)]
mod tests {

    use chrono::NaiveDate;

    #[test]
    fn parsing_date() {
        let date = NaiveDate::parse_from_str("1964-07-05", "%Y-%m-%d");
        assert_eq!(date, Ok(NaiveDate::from_ymd(1964,7,5)));
    }
}
