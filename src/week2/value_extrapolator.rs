use num::Integer;
use std::io::BufRead;

pub fn extrapolate_from_file(path: std::ffi::OsString) -> Vec<i64> {
    extrapolate_from_input(clio::Input::new(&path).unwrap())
}

pub fn reverse_extrapolate_from_file(path: std::ffi::OsString) -> Vec<i64> {
    reverse_extrapolate_from_input(clio::Input::new(&path).unwrap())
}

fn extrapolate_from_input(input: clio::Input) -> Vec<i64> {
    let values_list = get_values_from_input(input);
    let mut extrapolated_values = Vec::new();
    for values in values_list {
        extrapolated_values.push(get_extrapolated_value(values));
    }
    extrapolated_values
}

fn reverse_extrapolate_from_input(input: clio::Input) -> Vec<i64> {
    let values_list = get_values_from_input(input);
    let mut extrapolated_values = Vec::new();
    for mut values in values_list {
        values.reverse();
        extrapolated_values.push(get_extrapolated_value(values));
    }
    extrapolated_values
}

fn get_extrapolated_value<T: Integer+Copy>(values: Vec<T>) -> T {
    let all_deltas = generate_deltas_to_zero(values);
    let mut delta_iter = all_deltas.iter().rev();
    delta_iter.next();
    let mut last_value = num::zero();
    for current_delta in delta_iter{
        last_value = last_value + *current_delta.last().unwrap_or(&num::zero());
    }
    last_value
}

fn generate_deltas_to_zero<T: Integer+Copy>(values: Vec<T>) ->Vec<Vec<T>>{
    let mut all_deltas = Vec::new();
    all_deltas.push(values);
    while let Some(current_delta) = all_deltas.last() {
        if !current_delta
            .iter()
            .map(|a| a.is_zero())
            .reduce(|a, b| a && b)
            .unwrap_or(true)
        {
            let mut deltas = Vec::new();
            for num_pair in current_delta.iter().zip(current_delta.as_slice()[1..].iter()) {
                deltas.push(num_pair.1.sub(*num_pair.0));
            }
            all_deltas.push(deltas);
        }else{
            break;
        }
    }
    all_deltas
}

fn get_values_from_input(input: clio::Input) -> Vec<Vec<i64>> {
    let mut values_list = Vec::new();
    let input = std::io::BufReader::new(input);
    let mut lines = input.lines();
    while let Some(Ok(line)) = lines.next() {
        let mut values = Vec::new();
        let number_strs = line.split_ascii_whitespace();
        for number_str in number_strs {
            match number_str.parse::<i64>() {
                Ok(value) => values.push(value),
                Err(_) => eprint!("Could not parse this string into an i64: {}", number_str),
            }
        }
        values_list.push(values);
    }
    values_list
}
