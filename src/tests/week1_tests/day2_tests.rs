use crate::week1::cube_game::{get_all_minimum_dice_sets_from_path, get_all_valid_game_ids_from_path, get_power_of_dice_set};

#[test]
fn day2_1_test1(){
    assert_eq!(8, get_all_valid_game_ids_from_path("inputs/week1/input_021_test".into()).into_iter().sum());
}

#[test]
fn day2_1_final(){
    assert_eq!(2061, get_all_valid_game_ids_from_path("inputs/week1/input_02".into()).into_iter().sum());
}

#[test]
fn day2_2_test1(){
    assert_eq!(2286, get_all_minimum_dice_sets_from_path("inputs/week1/input_021_test".into()).iter().map(|dice|get_power_of_dice_set(dice)).sum());
}

#[test]
fn day2_2_final(){
    assert_eq!(72596, get_all_minimum_dice_sets_from_path("inputs/week1/input_02".into()).iter().map(|dice|get_power_of_dice_set(dice)).sum());
}