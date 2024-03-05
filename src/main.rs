use regex::Regex;
use std::io::{BufRead, BufReader};

fn main() {
        for arg in std::env::args_os().skip(1) {
        let input = clio::Input::new(&arg).unwrap();
        let calibration_sum = get_calibration_sums(input);
        println!("Sum of all calibration values in {arg:?} is {calibration_sum}");
    }
}


