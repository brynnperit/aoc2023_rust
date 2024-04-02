use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::HashMap,
    io::BufRead,
};

pub fn get_calibration_sums_from_path(path: std::ffi::OsString, use_words_for_digits: bool) -> i32 {
    let input = clio::Input::new(&path).unwrap();
    let calibration_sum = get_calibration_sums(input, use_words_for_digits);
    calibration_sum
}

pub fn get_calibration_sums(input: clio::Input, use_words_for_digits: bool) -> i32 {
    let mut calibration_sum = 0;
    let input = std::io::BufReader::new(input);
    for line in input.lines().map(|line| line.unwrap()) {
        calibration_sum += get_calibration_value(&line, use_words_for_digits);
    }
    calibration_sum
}

static DIGIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9])").unwrap());
static DIGIT_REGEX_INCLUDE_WORDS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap());

pub fn get_calibration_value(line: &str, use_words_for_digits: bool) -> i32 {
    let digit_regex = if use_words_for_digits {
        &DIGIT_REGEX_INCLUDE_WORDS
    } else {
        &DIGIT_REGEX
    };
    let mut remaining_line = line;
    let mut regex_match = digit_regex.find(remaining_line);
    let start_digit = get_digit_from_string(regex_match.unwrap().as_str());
    let mut end_digit = start_digit;
    while regex_match.is_some() {
        end_digit = get_digit_from_string(regex_match.unwrap().as_str());
        remaining_line = &remaining_line[regex_match.unwrap().start() + 1..];
        regex_match = digit_regex.find(remaining_line);
    }

    let calibration_value = (start_digit * 10) + end_digit;
    calibration_value
}

static DIGIT_WORDS: Lazy<HashMap<&str, i32>> = Lazy::new(|| {
    HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});

fn get_digit_from_string(digit_string: &str) -> i32 {
    match digit_string.parse::<i32>() {
        Ok(num) => num,
        Err(_) => *DIGIT_WORDS.get(digit_string).unwrap(),
    }
}
