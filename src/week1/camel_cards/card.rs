#[derive(Copy, Clone)]
pub struct Card {
    rank: u8,
}

impl Card {
    pub const WILDCARD_VALUE:u8=11;

    pub fn new(card_type: char) -> Option<Self> {
        let rank = match card_type {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            other => other.to_digit(10)?.try_into().ok()?,
        };
        Some(Card { rank })
    }
    pub fn get_rank(&self) -> u8 {
        self.rank
    }

    pub fn get_rank_wildcard(&self) -> u8 {
        if self.rank == Self::WILDCARD_VALUE {
            1
        } else {
            self.rank
        }
    }
}