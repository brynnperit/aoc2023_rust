use std::cmp::Ordering;

use super::{camel_cards_rules::CamelCardsRules, card::Card, card_hand_type::CardHandType};

pub struct CardHand {
    cards: Vec<Card>,
    hand_type: CardHandType,
    rules: CamelCardsRules,
    bid: u64,
}

impl CardHand {
    pub const CARD_HAND_SIZE:u8=5;

    pub fn from_str(line: &str, rules: CamelCardsRules) -> Option<Self> {
        let mut line_parts = line.split_ascii_whitespace();
        let hand_part = line_parts.next()?.chars();
        let bid_part = line_parts.next()?;
        let mut cards = Vec::with_capacity(5);
        for hand_char in hand_part {
            cards.push(Card::new(hand_char)?)
        }
        if cards.len() != 5 {
            return None;
        }
        let bid = bid_part.parse().ok()?;
        let hand_type = CardHandType::from_cards(&cards, rules);
        Some(CardHand {
            cards,
            hand_type,
            rules,
            bid,
        })
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        if self.rules != other.rules {
            panic!("Cannot compare wildcard hands to non-wildcard hands")
        }
        match self
            .hand_type
            .get_numerical_rank()
            .cmp(&other.hand_type.get_numerical_rank())
        {
            Ordering::Equal => {
                for pair in self.cards.iter().zip(other.cards.iter()) {
                    match self.rules {
                        CamelCardsRules::Standard => {
                            match pair.0.get_rank().cmp(&pair.1.get_rank()) {
                                Ordering::Equal => (),
                                other => return other,
                            }
                        }
                        CamelCardsRules::Wildcard => {
                            match pair.0.get_rank_wildcard().cmp(&pair.1.get_rank_wildcard()) {
                                Ordering::Equal => (),
                                other => return other,
                            }
                        }
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }

    pub fn get_bid(&self)->u64{
        self.bid
    }
}