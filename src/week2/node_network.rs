use network::Network;
use network_instruction::NetworkInstruction;

mod network;
mod network_instruction;

pub fn steps_to_navigate_network_in_file(path: std::ffi::OsString) -> u64 {
    steps_to_navigate_network_in_input(clio::Input::new(&path).unwrap())
}

pub fn steps_to_navigate_network_in_file_simul(path: std::ffi::OsString) -> u64 {
    steps_to_navigate_network_in_input_simul(clio::Input::new(&path).unwrap())
}

fn steps_to_navigate_network_in_input(input: clio::Input) -> u64 {
    let input = std::io::BufReader::new(input);
    let mut line_iter = std::io::BufRead::lines(input).map(|line| line.unwrap());
    let instruction_line = line_iter.next().unwrap();
    let instructions = NetworkInstruction::vector_from_line(&instruction_line);
    line_iter.next();
    let network = Network::from_iterator(line_iter);
    let mut steps = 0;
    let mut node = network.get_node("AAA");
    while node.borrow().get_name() != "ZZZ" {
        for instruction in instructions.iter() {
            let new_node = node.borrow().get_linked(*instruction).upgrade().unwrap();
            node = new_node;
            steps += 1;
        }
    }
    steps
}

fn steps_to_navigate_network_in_input_simul(input: clio::Input) -> u64 {
    let input = std::io::BufReader::new(input);
    let mut line_iter = std::io::BufRead::lines(input).map(|line| line.unwrap());
    let instruction_line = line_iter.next().unwrap();
    let instructions = NetworkInstruction::vector_from_line(&instruction_line);
    line_iter.next();
    let network = Network::from_iterator(line_iter);
    let mut steps = Vec::new();
    for start_node in network.all_node_names() {
        if start_node.ends_with('A') {
            let mut node = network.get_node(start_node);
            let mut step = 0;
            while !node.borrow().get_name().ends_with('Z') {
                for instruction in instructions.iter() {
                    let new_node = node.borrow().get_linked(*instruction).upgrade().unwrap();
                    node = new_node;
                    step += 1;
                    if node.borrow().get_name().ends_with('Z'){
                        break;
                    }
                }
            }
            steps.push(step);
        }
    }
    steps
        .into_iter()
        .reduce(|first, second| num::integer::lcm(first,second))
        .unwrap_or_default()
}
