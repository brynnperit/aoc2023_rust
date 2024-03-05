use std::io::{BufRead, BufReader};
use regex::Regex;
use once_cell::sync::Lazy;

pub fn get_calibration_sums(
    input: clio::Input,
) -> i32 {
    let mut calibration_sum = 0;
    let input = BufReader::new(input);
    for line in input
        .lines()
        .map(|line| line.unwrap_or_else(|_| panic!("Couldn't read line")))
    {
        calibration_sum += get_calibration_value(line);
    }
    calibration_sum
}

pub fn get_calibration_value(line: String) -> i32 {
    static START_DIGIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.*?([0-9])").unwrap());
    static END_DIGIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9])[^0-9]*?$").unwrap());
    let start_digit: i32 = START_DIGIT_REGEX
        .captures(&line)
        .unwrap_or_else(|| panic!("Digit must be present in {line}"))
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let end_digit: i32 = END_DIGIT_REGEX
        .captures(&line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let calibration_value = (start_digit * 10) + end_digit;
    calibration_value
}