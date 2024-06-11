use std::io::{BufReader, Lines};

pub fn get_maps(line_iter: &mut Lines<BufReader<clio::Input>>) -> Vec<SourceTargetMap> {
    let mut map_vec = Vec::new();
    while let Some(source_target_map) = SourceTargetMap::from_str(
        line_iter
            .next()
            .unwrap_or(Ok("".to_string()))
            .unwrap_or("".to_string())
            .as_str(),
    ) {
        map_vec.push(source_target_map);
    }
    map_vec
}

pub struct SourceTargetMap {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl SourceTargetMap {
    pub fn from_str(line: &str) -> Option<SourceTargetMap> {
        let mut line_iter = line.split_ascii_whitespace();
        let destination_range_start = line_iter.next()?.parse::<usize>().ok()?;
        let source_range_start = line_iter.next()?.parse::<usize>().ok()?;
        let range_length = line_iter.next()?.parse::<usize>().ok()?;
        Some(SourceTargetMap {
            destination_range_start: destination_range_start,
            source_range_start: source_range_start,
            range_length: range_length,
        })
    }

    pub fn map_number(&self, number: usize) -> Option<usize> {
        if number >= self.source_range_start && number < self.source_range_start + self.range_length
        {
            return Some(number - self.source_range_start + self.destination_range_start);
        }
        None
    }

    pub fn get_source_range_start(&self) -> usize{
        self.source_range_start
    }

    pub fn get_range_length(&self) -> usize{
        self.range_length
    }
}