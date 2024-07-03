#[derive(Clone, Copy)]
pub enum NetworkInstruction {
    L,
    R,
}

impl NetworkInstruction {
    pub fn vector_from_line(line: &str) -> Vec<Self> {
        let mut instructions = Vec::with_capacity(line.len());
        for instruction_char in line.chars() {
            instructions.push(Self::from_char(instruction_char));
        }
        instructions
    }

    pub fn from_char(instruction_char: char) -> Self {
        match instruction_char {
            'L' => Self::L,
            'R' => Self::R,
            other => panic!("There is no instruction mapping for {}", other),
        }
    }
}
