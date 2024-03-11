use crate::get_calibration_sums;

#[test]
fn day1_1_test1(){
    assert_eq!(142, get_calibration_sums_from_path("inputs/week1/input_011_test".into(), false));
}

#[test]
fn day1_1_test2(){
    assert_eq!(54927, get_calibration_sums_from_path("inputs/week1/input_01".into(), false));
}

#[test]
fn day1_2_test1(){
    assert_eq!(281, get_calibration_sums_from_path("inputs/week1/input_012_test".into(), true));
}

#[test]
fn day1_2_test2(){
    assert_eq!(54581, get_calibration_sums_from_path("inputs/week1/input_01".into(), true));
}

fn get_calibration_sums_from_path(arg:std::ffi::OsString, use_words_for_digits:bool) -> i32 {
    let input = clio::Input::new(&arg).unwrap();
    let calibration_sum = get_calibration_sums(input,use_words_for_digits);
    calibration_sum
    //println!("Sum of all calibration values in {arg:?} is {calibration_sum}");
}
