
use chrono::*;

#[allow(dead_code)]
pub fn tutorial_1() {
    let start = UTC.ymd(2006, 1, 1);
    let end = UTC.ymd(2010, 12, 31);
    let format = "%Y %B %d";
    println!("Processing for period between {} and {}",
             start.format(format),
             end.format(format));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tutorial_1_test() {
        tutorial_1();
    }
}
