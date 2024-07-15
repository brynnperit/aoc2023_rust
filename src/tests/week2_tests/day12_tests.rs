use crate::week2::spring_arrangements;

#[test]
fn day12_1_test1() {
    assert_eq!(
        21_u64,
        spring_arrangements::find_all_arrangements_in_file(
            "inputs/week2/input_121_test".into(),
            spring_arrangements::Repetition::None
        )
        .into_iter()
        .sum()
    )
}

#[test]
fn day12_1_test2() {
    assert_eq!(
        7674_u64,
        spring_arrangements::find_all_arrangements_in_file(
            "inputs/week2/input_12".into(),
            spring_arrangements::Repetition::None
        )
        .into_iter()
        .sum()
    )
}

#[test]
fn day12_2_test1() {
    assert_eq!(
        525152_u64,
        spring_arrangements::find_all_arrangements_in_file(
            "inputs/week2/input_121_test".into(),
            spring_arrangements::Repetition::LineRepetitions(5)
        )
        .into_iter()
        .sum()
    )
}

#[test]
fn day12_2_test2() {
    assert_eq!(
        4443895258186_u64,
        spring_arrangements::find_all_arrangements_in_file(
            "inputs/week2/input_12".into(),
            spring_arrangements::Repetition::LineRepetitions(5)
        )
        .into_iter()
        .sum()
    )
}
