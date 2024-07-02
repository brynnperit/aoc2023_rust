use super::{camel_cards_rules::CamelCardsRules, card::Card, card_hand::CardHand};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CardHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardHandType {
    pub fn from_cards(cards: &[Card], rules: CamelCardsRules) -> Self {
        let mut ranks = Vec::new();
        let mut counts = Vec::new();
        Self::get_ranks_and_counts(cards, rules, &mut ranks, &mut counts);
        match ranks.len() {
            5 => CardHandType::HighCard,
            4 => CardHandType::OnePair,
            3 => {
                let max_count = *(counts.iter().max().unwrap());
                match max_count {
                    3 => CardHandType::ThreeOfAKind,
                    2 => CardHandType::TwoPair,
                    _ => panic!(),
                }
            }
            2 => {
                let max_count = *(counts.iter().max().unwrap());
                match max_count {
                    4 => CardHandType::FourOfAKind,
                    3 => CardHandType::FullHouse,
                    _ => panic!(),
                }
            }
            1 => CardHandType::FiveOfAKind,
            _ => panic!(),
        }
    }

    pub fn get_numerical_rank(&self) -> u8 {
        *self as u8
    }

    fn get_ranks_and_counts(
        cards: &[Card],
        rules: CamelCardsRules,
        ranks: &mut Vec<u8>,
        counts: &mut Vec<u8>,
    ) {
        let mut cards: Vec<Card> = cards.iter().map(|card| card.clone()).collect();
        match rules {
            CamelCardsRules::Standard => cards.sort_unstable_by_key(Card::get_rank),
            CamelCardsRules::Wildcard => cards.sort_unstable_by_key(|card|std::cmp::Reverse(card.get_rank_wildcard())),
        }
        let mut previous_card_rank = cards[0].get_rank();
        let mut count = 1;
        for card in &cards[1..] {
            if card.get_rank() == previous_card_rank {
                count += 1;
            } else {
                ranks.push(previous_card_rank);
                counts.push(count);
                previous_card_rank = card.get_rank();
                count = 1;
            }
        }
        match rules {
            CamelCardsRules::Standard => {
                ranks.push(previous_card_rank);
                counts.push(count);
            }
            CamelCardsRules::Wildcard => {
                //J is smallest card under wildcard rules, J is the last one evaluated;
                //If all Js then that's 5 of a kind, otherwise add the number of Js to the largest existing card count
                if previous_card_rank == Card::WILDCARD_VALUE && count != CardHand::CARD_HAND_SIZE{
                    let max_index = counts.iter().enumerate().max_by_key(|(_idx, &val)| val).unwrap().0;
                    counts[max_index] += count;
                }else{
                    ranks.push(previous_card_rank);
                    counts.push(count);
                }
            },
        }
    }
}