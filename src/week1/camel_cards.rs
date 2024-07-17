use camel_cards_rules::CamelCardsRules;
use card_hand::CardHand;

mod camel_cards_rules;
mod card;
mod card_hand;
mod card_hand_type;

pub fn get_winnings_from_file(path: std::ffi::OsString) -> Vec<u64> {
    get_winnings_from_input(clio::Input::new(&path).unwrap(), CamelCardsRules::Standard)
}

pub fn get_wildcard_winnings_from_file(path: std::ffi::OsString) -> Vec<u64> {
    get_winnings_from_input(clio::Input::new(&path).unwrap(), CamelCardsRules::Wildcard)
}

fn get_winnings_from_input(input: clio::Input, rules: CamelCardsRules) -> Vec<u64> {
    let mut card_winnings = Vec::new();
    let mut card_hands = get_card_hands_from_input(input, rules);
    card_hands.sort_unstable_by(CardHand::cmp);
    let mut hand_rank = 1;
    for hand in card_hands {
        card_winnings.push(hand.get_bid() * hand_rank);
        hand_rank += 1;
    }
    card_winnings
}

fn get_card_hands_from_input(input: clio::Input, rules: CamelCardsRules) -> Vec<CardHand> {
    let mut hands = Vec::new();
    let input = std::io::BufReader::new(input);
    for line in std::io::BufRead::lines(input).map_while(Result::ok) {
        if let Some(card) = CardHand::from_str(&line, rules) {
            hands.push(card);
        }
    }
    hands
}
