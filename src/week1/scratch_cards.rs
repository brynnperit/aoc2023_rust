use std::io::BufRead;

pub fn get_all_card_winnings_from_path(path: std::ffi::OsString) -> Vec<u32> {
    get_all_card_winnings_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_total_card_count_from_path(path: std::ffi::OsString) -> usize {
    get_total_card_count_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_all_card_winnings_from_input(input: clio::Input) -> Vec<u32> {
    let mut card_winnings = Vec::new();
    let input = std::io::BufReader::new(input);
    for line in input.lines().map_while(Result::ok) {
        match ScratchCard::from_str(&line) {
                Some(card) => card_winnings.push(card.get_winnings()),
                None => panic!("This line was not formatted properly: {}", line),
        }
    }
    card_winnings
}

pub fn get_total_card_count_from_input(input: clio::Input) -> usize {
    let mut total_count = 0;
    let input = std::io::BufReader::new(input);
    let mut all_cards = Vec::new();
    for line in input.lines().map_while(Result::ok) {
        match ScratchCard::from_str(&line) {
            Some(card) => all_cards.push(card),
            None => panic!("This line was not formatted properly: {}", line),
        }
    }
    let mut all_card_winning_numbers = Vec::new();
    let mut card_counts = Vec::new();
    for card in all_cards {
        all_card_winning_numbers.push(card.get_winning_number_count());
        card_counts.push(1_usize);
    }
    for card_index in 0..all_card_winning_numbers.len() {
        let win_count = all_card_winning_numbers[card_index];
        let card_count = card_counts[card_index];
        total_count += card_count;
        for won_cards in card_counts
            .iter_mut()
            .take(card_index + win_count + 1)
            .skip(card_index + 1)
        {
            *won_cards += card_count;
        }
    }
    total_count
}

struct ScratchCard {
    winning_numbers: Vec<u32>,
    contained_numbers: Vec<u32>,
}

impl ScratchCard {
    fn get_winnings(&self) -> u32 {
        let mut winnings = 0;
        for contained_number in self.contained_numbers.as_slice() {
            if self.winning_numbers.contains(contained_number) {
                match winnings {
                    0 => winnings += 1,
                    _ => winnings *= 2,
                }
            }
        }
        winnings
    }
    fn get_winning_number_count(&self) -> usize {
        let mut winnings = 0;
        for contained_number in self.contained_numbers.as_slice() {
            if self.winning_numbers.contains(contained_number) {
                winnings += 1;
            }
        }
        winnings
    }
    fn from_str(line: &str) -> Option<ScratchCard> {
        let mut split_line_iter = line.split_ascii_whitespace();
        split_line_iter.next();
        split_line_iter.next();
        let mut winning_numbers = Vec::new();
        let mut contained_numbers = Vec::new();
        while let Ok(winning_number) = split_line_iter.next().unwrap().parse::<u32>() {
            winning_numbers.push(winning_number);
        }
        while let Ok(contained_number) = split_line_iter.next().unwrap_or("").parse::<u32>() {
            contained_numbers.push(contained_number);
        }
        Some(ScratchCard {
            winning_numbers,
            contained_numbers,
        })
    }
}
