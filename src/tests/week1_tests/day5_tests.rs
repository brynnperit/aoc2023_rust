use crate::week1::seed_maps;

#[test]
fn day5_1_test1(){
    assert_eq!(35_usize, seed_maps::get_location_numbers_from_seed_file("inputs/week1/input_051_test".into()).into_iter().min().unwrap_or_default());
}

#[test]
fn day5_1_test2(){
    assert_eq!(525792406_usize, seed_maps::get_location_numbers_from_seed_file("inputs/week1/input_05".into()).into_iter().min().unwrap_or_default());
}

#[test]
fn day5_2_test1(){
    assert_eq!(46_usize, seed_maps::get_location_numbers_from_seed_file_as_ranges("inputs/week1/input_051_test".into()).into_iter().min().unwrap_or_default());
}

#[test]
fn day5_2_test2(){
    assert_eq!(79004094_usize, seed_maps::get_location_numbers_from_seed_file_as_ranges("inputs/week1/input_05".into()).into_iter().min().unwrap_or_default());
}