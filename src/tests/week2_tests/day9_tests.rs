use crate::week2::value_extrapolator;

#[test]
fn day9_1_test1() {
    assert_eq!(
        114_i64,
        value_extrapolator::extrapolate_from_file("inputs/week2/input_091_test".into()).into_iter().sum()
    );
}

#[test]
fn day9_1_test2() {
    assert_eq!(
        1904165718_i64,
        value_extrapolator::extrapolate_from_file("inputs/week2/input_09".into()).into_iter().sum()
    );
}

#[test]
fn day9_2_test1() {
    assert_eq!(
        2_i64,
        value_extrapolator::reverse_extrapolate_from_file("inputs/week2/input_091_test".into()).into_iter().sum()
    );
}

#[test]
fn day9_2_test2() {
    assert_eq!(
        964_i64,
        value_extrapolator::reverse_extrapolate_from_file("inputs/week2/input_09".into()).into_iter().sum()
    );
}
