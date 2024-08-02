use crate::week3::light_grid::LightGrid;

#[test]
fn day16_1_test1() {
    let light_grid = LightGrid::from_file("inputs/week3/input_161_test".into());
    assert_eq!(46_u64, light_grid.energized_tiles_from_default_entry(),)
}

#[test]
fn day16_1_test2() {
    let light_grid = LightGrid::from_file("inputs/week3/input_16".into());
    assert_eq!(7562_u64, light_grid.energized_tiles_from_default_entry(),)
}

#[test]
fn day16_2_test1() {
    let light_grid = LightGrid::from_file("inputs/week3/input_161_test".into());
    assert_eq!(51_u64, light_grid.max_energized_tiles(),)
}

#[test]
fn day16_2_test2() {
    let light_grid = LightGrid::from_file("inputs/week3/input_16".into());
    assert_eq!(7793_u64, light_grid.max_energized_tiles(),)
}
