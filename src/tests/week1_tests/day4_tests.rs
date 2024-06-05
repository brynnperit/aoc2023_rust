use crate::week1::scratch_cards;

#[test]
fn day4_1_test1(){
    assert_eq!(13_u32, scratch_cards::get_all_card_winnings_from_path("inputs/week1/input_041_test".into()).iter().sum());
}

#[test]
fn day4_1_test2(){
    assert_eq!(23678_u32, scratch_cards::get_all_card_winnings_from_path("inputs/week1/input_04".into()).iter().sum());
}

#[test]
fn day4_2_test1(){
    assert_eq!(30_usize, scratch_cards::get_total_card_count_from_path("inputs/week1/input_042_test".into()));
}

#[test]
fn day4_2_test2(){
    assert_eq!(15455663_usize, scratch_cards::get_total_card_count_from_path("inputs/week1/input_04".into()));
}