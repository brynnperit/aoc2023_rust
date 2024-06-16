use crate::week1::boat_race;

#[test]
fn day6_1_test1(){
    assert_eq!(288, boat_race::get_count_of_winning_strategies_from_file("inputs/week1/input_061_test".into()).into_iter().reduce(|a,b|a * b).unwrap_or_default());
}

#[test]
fn day6_1_test2(){
    assert_eq!(2065338, boat_race::get_count_of_winning_strategies_from_file("inputs/week1/input_06".into()).into_iter().reduce(|a,b|a * b).unwrap_or_default());
}

#[test]
fn day6_2_test1(){
    assert_eq!(71503, boat_race::get_count_of_winning_strategies_from_file_as_single_race("inputs/week1/input_061_test".into()));
}

#[test]
fn day6_2_test2(){
    assert_eq!(34934171, boat_race::get_count_of_winning_strategies_from_file_as_single_race("inputs/week1/input_06".into()));
}