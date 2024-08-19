use crate::week3::crucible_grid::CrucibleGrid;

#[test]
fn day17_1_test1() {
    let crucible_grid = CrucibleGrid::from_file("inputs/week3/input_171_test".into());
    assert_eq!(
        102_u64,
        crucible_grid
            .find_minimum_heat_loss_route(1, 3)
            .get_elapsed_cost(),
    )
}

#[test]
fn day17_1_test2() {
    let crucible_grid = CrucibleGrid::from_file("inputs/week3/input_17".into());
    assert_eq!(
        1256_u64,
        crucible_grid
            .find_minimum_heat_loss_route(1, 3)
            .get_elapsed_cost(),
    )
}

#[test]
fn day17_2_test1() {
    let crucible_grid = CrucibleGrid::from_file("inputs/week3/input_171_test".into());
    assert_eq!(
        94_u64,
        crucible_grid
            .find_minimum_heat_loss_route(4, 10)
            .get_elapsed_cost(),
    )
}

#[test]
fn day17_2_test2() {
    let crucible_grid = CrucibleGrid::from_file("inputs/week3/input_17".into());
    assert_eq!(
        1382_u64,
        crucible_grid
            .find_minimum_heat_loss_route(4, 10)
            .get_elapsed_cost(),
    )
}
