use crate::{shared_libs::direction::Direction, week2::reflector_rocks::ReflectorRocks};

#[test]
fn day14_1_test1() {
    let mut rock_grid = ReflectorRocks::from_file("inputs/week2/input_141_test".into());
    rock_grid.roll(&Direction::North);
    assert_eq!(
        136_u64,
        rock_grid.get_loads(&Direction::North).into_iter().sum(),
    )
}

#[test]
fn day14_1_test2() {
    let mut rock_grid = ReflectorRocks::from_file("inputs/week2/input_14".into());
    rock_grid.roll(&Direction::North);
    assert_eq!(
        107142_u64,
        rock_grid.get_loads(&Direction::North).into_iter().sum(),
    )
}

#[test]
fn day14_2_test1() {
    let mut rock_grid = ReflectorRocks::from_file("inputs/week2/input_141_test".into());
    rock_grid.spin_cycle(
        vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ],
        1000000000,
    );
    assert_eq!(
        64_u64,
        rock_grid.get_loads(&Direction::North).into_iter().sum(),
    )
}

#[test]
fn day14_2_test2() {
    let mut rock_grid = ReflectorRocks::from_file("inputs/week2/input_14".into());
    rock_grid.spin_cycle(
        vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ],
        1000000000,
    );
    assert_eq!(
        104815_u64,
        rock_grid.get_loads(&Direction::North).into_iter().sum(),
    )
}
