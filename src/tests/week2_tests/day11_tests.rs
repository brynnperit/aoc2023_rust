use crate::week2::galaxy_mapper;

#[test]
fn day11_1_test1() {
    assert_eq!(
        374_u64,
        galaxy_mapper::shortest_distances_between_galaxies_in_file(
            "inputs/week2/input_111_test".into(),
            galaxy_mapper::ExpansionFactor::Multiply(2)
        )
        .into_iter()
        .map(|trio| trio.2)
        .sum()
    );
}

#[test]
fn day11_1_test2() {
    assert_eq!(
        10292708_u64,
        galaxy_mapper::shortest_distances_between_galaxies_in_file(
            "inputs/week2/input_11".into(),
            galaxy_mapper::ExpansionFactor::Multiply(2)
        )
        .into_iter()
        .map(|trio| trio.2)
        .sum()
    );
}

#[test]
fn day11_2_test1() {
    assert_eq!(
        82000210_u64,
        galaxy_mapper::shortest_distances_between_galaxies_in_file(
            "inputs/week2/input_111_test".into(),
            galaxy_mapper::ExpansionFactor::Multiply(1000000)
        )
        .into_iter()
        .map(|trio| trio.2)
        .sum()
    );
}

#[test]
fn day11_2_test2() {
    assert_eq!(
        790194712336_u64,
        galaxy_mapper::shortest_distances_between_galaxies_in_file(
            "inputs/week2/input_11".into(),
            galaxy_mapper::ExpansionFactor::Multiply(1000000)
        )
        .into_iter()
        .map(|trio| trio.2)
        .sum()
    );
}
