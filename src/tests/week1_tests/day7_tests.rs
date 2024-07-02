use crate::week1::camel_cards;

#[test]
fn day7_1_test1(){
    assert_eq!(6440_u64, camel_cards::get_winnings_from_file("inputs/week1/input_071_test".into()).into_iter().sum());
}

#[test]
fn day7_1_test2(){
    assert_eq!(248422077_u64, camel_cards::get_winnings_from_file("inputs/week1/input_07".into()).into_iter().sum());
}

#[test]
fn day7_2_test1(){
    assert_eq!(5905_u64, camel_cards::get_wildcard_winnings_from_file("inputs/week1/input_071_test".into()).into_iter().sum());
}

#[test]
fn day7_2_test2(){
    assert_eq!(249817836_u64, camel_cards::get_wildcard_winnings_from_file("inputs/week1/input_07".into()).into_iter().sum());
}