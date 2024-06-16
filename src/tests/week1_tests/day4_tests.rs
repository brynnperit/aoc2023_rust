use crate::week1::scratch_cards;

#[test]
fn day4_1_test1(){
    assert_eq!(13, scratch_cards::get_all_card_winnings_from_path("inputs/week1/input_041_test".into()).into_iter().sum::<u32>());
}

#[test]
fn day4_1_test2(){
    assert_eq!(23678, scratch_cards::get_all_card_winnings_from_path("inputs/week1/input_04".into()).iter().sum::<u32>());
}

#[test]
fn day4_2_test1(){
    assert_eq!(30, scratch_cards::get_total_card_count_from_path("inputs/week1/input_042_test".into()));
}

#[test]
fn day4_2_test2(){
    assert_eq!(15455663, scratch_cards::get_total_card_count_from_path("inputs/week1/input_04".into()));
}