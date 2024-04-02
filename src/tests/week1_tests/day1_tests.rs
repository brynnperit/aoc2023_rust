use crate::week1::get_calibration::get_calibration_sums_from_path;

#[test]
fn day1_1_test1(){
    assert_eq!(142, get_calibration_sums_from_path("inputs/week1/input_011_test".into(), false));
}

#[test]
fn day1_1_final(){
    assert_eq!(54927, get_calibration_sums_from_path("inputs/week1/input_01".into(), false));
}

#[test]
fn day1_2_test1(){
    assert_eq!(281, get_calibration_sums_from_path("inputs/week1/input_012_test".into(), true));
}

#[test]
fn day1_2_final(){
    assert_eq!(54581, get_calibration_sums_from_path("inputs/week1/input_01".into(), true));
}
