
use chrono::*;

#[allow(dead_code)]
pub fn tutorial_1() {
    let start = UTC.ymd(2006, 1, 1).and_hms(16, 0, 0);
    let end = UTC.ymd(2010, 12, 31).and_hms(16, 0, 0);
    let date_format = "%Y %B %d";
    let time_format = "%H:%S";
    println!("Processing for period between {} and {} (at {}).",
             start.format(date_format),
             end.format(date_format),
             start.format(time_format));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tutorial_1_test() {
        tutorial_1();
    }
}
