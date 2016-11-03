extern crate clap;
extern crate chrono;

use clap::{Arg, App};
use chrono::*;

fn main() {
    let matches = App::new("Quant Software ")
        .version("0.1")
        .author("Marek Dudek")
        .arg(Arg::with_name("from").long("from").takes_value(true))
        .get_matches();

    let from = matches.value_of("from").unwrap_or("1964-07-05 01:01:01 +0900");
    let from_date = DateTime::parse_from_str(from, "%Y-%m-%d %H:%M:%S %z");
    println!("from: {:?}", from_date);
}
