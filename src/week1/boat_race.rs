pub fn get_count_of_winning_strategies_from_file(path: std::ffi::OsString) -> Vec<u64> {
    get_count_of_winning_strategies_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_count_of_winning_strategies_from_file_as_single_race(path: std::ffi::OsString) -> u64 {
    get_count_of_winning_strategies_from_input_as_single_race(clio::Input::new(&path).unwrap())
}

fn get_count_of_winning_strategies_from_input(input: clio::Input) -> Vec<u64> {
    let input = std::io::BufReader::new(input);
    let mut line_iter = std::io::BufRead::lines(input);
    let time_line = line_iter.next().unwrap().unwrap();
    let mut time_line_iter = time_line.split_ascii_whitespace();
    time_line_iter.next();
    let mut race_times = Vec::new();
    for time_line in time_line_iter {
        if let Ok(race_time) = time_line.parse() {
            race_times.push(race_time);
        }
    }

    let distance_line = line_iter.next().unwrap().unwrap();
    let mut distance_line_iter = distance_line.split_ascii_whitespace();
    distance_line_iter.next();
    let mut race_distances = Vec::new();
    for distance_line in distance_line_iter {
        if let Ok(race_distance) = distance_line.parse() {
            race_distances.push(race_distance);
        }
    }

    let mut boat_races = Vec::new();
    for (race_time, race_distance) in race_times.into_iter().zip(race_distances.into_iter()) {
        boat_races.push(BoatRace::new(race_time, race_distance));
    }
    get_count_of_winning_strategies_for_boat_races(boat_races)
}

fn get_count_of_winning_strategies_from_input_as_single_race(input: clio::Input) -> u64 {
    let input = std::io::BufReader::new(input);
    let mut line_iter = std::io::BufRead::lines(input);
    let time_line = line_iter.next().unwrap().unwrap();
    let mut time_line_iter = time_line.split_ascii_whitespace();
    time_line_iter.next();
    let race_time = time_line_iter
        .fold("".to_string(), |a, b| a + b)
        .parse()
        .unwrap();

    let distance_line = line_iter.next().unwrap().unwrap();
    let mut distance_line_iter = distance_line.split_ascii_whitespace();
    distance_line_iter.next();
    let race_distance = distance_line_iter
        .fold("".to_string(), |a, b| a + b)
        .parse()
        .unwrap();

    get_count_of_winning_strategies_for_boat_races(vec![BoatRace::new(race_time, race_distance)])
        .pop()
        .unwrap_or_default()
}

fn get_count_of_winning_strategies_for_boat_races(boat_races: Vec<BoatRace>) -> Vec<u64> {
    let mut counts = Vec::new();
    for boat_race in boat_races {
        let quad_a = 1_f64;
        let quad_b = -(boat_race.race_time as f64);
        let quad_c = (boat_race.record_distance + 1) as f64;
        let lower_bound =
            (-quad_b - (quad_b * quad_b - 4_f64 * quad_a * quad_c).sqrt()) / (2_f64 * quad_a);
        let upper_bound =
            (-quad_b + (quad_b * quad_b - 4_f64 * quad_a * quad_c).sqrt()) / (2_f64 * quad_a);
        let range_size = (upper_bound.floor() - lower_bound.ceil()) as u64 + 1;
        counts.push(range_size);
    }
    counts
}

struct BoatRace {
    race_time: u64,
    record_distance: u64,
}

impl BoatRace {
    fn new(race_time: u64, record_distance: u64) -> Self {
        BoatRace {
            race_time,
            record_distance,
        }
    }
}
