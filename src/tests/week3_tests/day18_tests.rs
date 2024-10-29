use crate::week3::lagoon::Lagoon;

#[test]
fn day18_1_test1() {
    let lagoon = Lagoon::from_plain_file("inputs/week3/input_181_test".into());
    assert_eq!(
        62_u64,
        lagoon.calculate_area(),
    )
}

#[test]
fn day18_1_test2() {
    let lagoon = Lagoon::from_plain_file("inputs/week3/input_18".into());
    assert_eq!(
        62573_u64,
        lagoon.calculate_area(),
    )
}

#[test]
fn day18_2_test1() {
    let lagoon = Lagoon::from_hex_file("inputs/week3/input_181_test".into());
    assert_eq!(
        952408144115_u64,
        lagoon.calculate_area(),
    )
}

#[test]
fn day18_2_test2() {
    let lagoon = Lagoon::from_hex_file("inputs/week3/input_18".into());
    assert_eq!(
        54662804037719_u64,
        lagoon.calculate_area(),
    )
}
