use crate::week2::pipe_mapper;

#[test]
fn day10_1_test1() {
    assert_eq!(
        4_u64,
        pipe_mapper::loop_length_in_file("inputs/week2/input_101_test".into()).div_ceil(2)
    );
}

#[test]
fn day10_1_test2() {
    assert_eq!(
        6890_u64,
        pipe_mapper::loop_length_in_file("inputs/week2/input_10".into()).div_ceil(2)
    );
}

#[test]
fn day10_2_test1() {
    assert_eq!(
        1_u64,
        pipe_mapper::enclosed_tiles_in_file("inputs/week2/input_101_test".into())
    );
}

#[test]
fn day10_2_test2() {
    assert_eq!(
        10_u64,
        pipe_mapper::enclosed_tiles_in_file("inputs/week2/input_102_test".into())
    );
}

#[test]
fn day10_2_test3() {
    assert_eq!(
        453_u64,
        pipe_mapper::enclosed_tiles_in_file("inputs/week2/input_10".into())
    );
}
