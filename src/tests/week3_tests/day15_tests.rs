use crate::week3::lens_sequence::LensSequence;

#[test]
fn day15_1_test1() {
    let lenses = LensSequence::from_file("inputs/week3/input_151_test".into());
    assert_eq!(
        1320_u64,
        lenses.get_hashes().into_iter().map(u64::from).sum(),
    )
}

#[test]
fn day15_1_test2() {
    let lenses = LensSequence::from_file("inputs/week3/input_15".into());
    assert_eq!(
        504036_u64,
        lenses.get_hashes().into_iter().map(u64::from).sum(),
    )
}

#[test]
fn day15_2_test1() {
    let lenses = LensSequence::from_file("inputs/week3/input_151_test".into());
    assert_eq!(145_u64, lenses.get_focusing_power(),)
}

#[test]
fn day15_2_test2() {
    let lenses = LensSequence::from_file("inputs/week3/input_15".into());
    assert_eq!(295719_u64, lenses.get_focusing_power(),)
}
