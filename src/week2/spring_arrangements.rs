use std::{io::BufRead, sync::mpsc, thread};

use spring_arrangement::SpringArrangement;

mod spring_arrangement;

pub fn find_all_arrangements_in_file(
    path: std::ffi::OsString,
    line_repeats: Repetition,
) -> Vec<u64> {
    find_all_arrangements_in_input(clio::Input::new(&path).unwrap(), line_repeats)
}

fn find_all_arrangements_in_input(input: clio::Input, line_repeats: Repetition) -> Vec<u64> {
    let threads = std::thread::available_parallelism().unwrap().get();
    let arrangements = get_all_arrangements_from_input(input, line_repeats);
    let mut arrangement_chunks = Vec::new();
    for _ in 0..threads {
        arrangement_chunks.push(Vec::new());
    }
    for (index, arrangement) in arrangements.into_iter().enumerate() {
        arrangement_chunks[index % threads].push(arrangement);
    }
    let (tx, rx) = mpsc::channel();
    for arrangement_chunk in arrangement_chunks {
        let tx = tx.clone();
        thread::spawn(move || {
            for arrangement in arrangement_chunk {
                tx.send(arrangement.get_number_of_possible_arrangements())
                    .unwrap();
            }
        });
    }
    drop(tx);
    rx.into_iter().collect()
}

fn get_all_arrangements_from_input(
    input: clio::Input,
    line_repeats: Repetition,
) -> Vec<SpringArrangement> {
    let input = std::io::BufReader::new(input);
    let mut spring_arrangements = Vec::new();
    for line in input.lines().map(|result| result.unwrap()) {
        spring_arrangements.push(SpringArrangement::from_str(&line, &line_repeats));
    }
    spring_arrangements
}

pub enum Repetition {
    LineRepetitions(u64),
    None,
}

#[cfg(test)]
pub mod spring_tests {
    use super::*;

    #[test]
    pub fn repetition_test() {
        let manual_repeat =
            SpringArrangement::from_str(".#?.#?.#?.#?.# 1,1,1,1,1", &Repetition::None);
        let auto_repeat = SpringArrangement::from_str(".# 1", &Repetition::LineRepetitions(5));
        assert!(Iterator::eq(
            manual_repeat.get_springs(),
            auto_repeat.get_springs()
        ));
        assert!(Iterator::eq(
            manual_repeat.get_damaged_section_sizes(),
            auto_repeat.get_damaged_section_sizes()
        ));
    }
}
