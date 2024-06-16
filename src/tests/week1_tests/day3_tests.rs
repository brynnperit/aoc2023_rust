use crate::week1::engine_gears;

#[test]
fn day3_1_test1(){
    assert_eq!(4361, engine_gears::get_all_part_numbers_from_path("inputs/week1/input_031_test".into()).into_iter().sum());
}

#[test]
fn day3_1_final(){
    assert_eq!(531932, engine_gears::get_all_part_numbers_from_path("inputs/week1/input_03".into()).into_iter().sum());
}

#[test]
fn day3_2_test1(){
    assert_eq!(467835, engine_gears::get_all_gear_ratios_from_path("inputs/week1/input_031_test".into()).into_iter().sum());
}

#[test]
fn day3_2_final(){
    assert_eq!(73646890, engine_gears::get_all_gear_ratios_from_path("inputs/week1/input_03".into()).into_iter().sum());
}