use enum_map::{Enum, EnumMap};
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::BufRead;

pub fn get_power_of_dice_set(cube_set: &EnumMap<DiceColor, i32>) -> i32 {
    cube_set
        .into_values()
        .reduce(|i1, i2| i1 * i2)
        .unwrap_or_default()
}

pub fn get_all_minimum_dice_sets_from_path(
    path: std::ffi::OsString,
) -> Vec<EnumMap<DiceColor, i32>> {
    get_all_minimum_dice_sets_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_all_valid_game_ids_from_path(path: std::ffi::OsString) -> Vec<i32> {
    get_all_valid_game_ids_from_input(clio::Input::new(&path).unwrap())
}

pub fn get_all_minimum_dice_sets_from_input(input: clio::Input) -> Vec<EnumMap<DiceColor, i32>> {
    let mut minimum_cube_sets = Vec::new();
    let input = std::io::BufReader::new(input);
    for line in input.lines().map(|line| line.unwrap()) {
        match get_minimum_dice_set_from_string(&line) {
            Some(dice_set) => minimum_cube_sets.push(dice_set),
            None => println!("Was expecting a valid dice set on line {line}"),
        }
    }
    minimum_cube_sets
}

pub fn get_all_valid_game_ids_from_input(input: clio::Input) -> Vec<i32> {
    let mut game_ids = Vec::new();
    let input = std::io::BufReader::new(input);
    for line in input.lines().map(|line| line.unwrap()) {
        if let Some(id) = get_id_of_valid_game_from_string(&line, DEFAULT_GAME_DICE_LIMITS) {
            game_ids.push(id);
        }
    }
    game_ids
}

static DEFAULT_GAME_DICE_LIMITS: [DiceLimit; 3] = [
    DiceLimit::new(DiceColor::Red, 12),
    DiceLimit::new(DiceColor::Green, 13),
    DiceLimit::new(DiceColor::Blue, 14),
];

#[derive(Clone, Copy)]
pub struct DiceLimit {
    color: DiceColor,
    count: i32,
}

impl DiceLimit {
    const fn new(color: DiceColor, count: i32) -> DiceLimit {
        DiceLimit { color, count }
    }
}

#[derive(Enum, Clone, Copy)]
pub enum DiceColor {
    Red,
    Green,
    Blue,
}

fn get_dice_color_from_string(color_string: &str) -> Option<DiceColor> {
    match color_string {
        "red" => Some(DiceColor::Red),
        "green" => Some(DiceColor::Green),
        "blue" => Some(DiceColor::Blue),
        _ => None,
    }
}

static GAME_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Game ([0-9]+):").unwrap());
static DICE_COUNT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([0-9]+) (red|green|blue)").unwrap());

fn get_id_of_valid_game_from_string(
    game_line: &str,
    valid_dice_limits: impl IntoIterator<Item = DiceLimit>,
) -> Option<i32> {
    let made_id = GAME_ID_REGEX
        .captures(game_line)?
        .get(1)?
        .as_str()
        .parse::<i32>()
        .ok()?;

    let dice_limits = get_minimum_dice_set_from_string(game_line)?;
    for valid_dice_limit in valid_dice_limits.into_iter() {
        if valid_dice_limit.count < dice_limits[valid_dice_limit.color] {
            return None;
        }
    }
    Some(made_id)
}

fn get_minimum_dice_set_from_string(game_line: &str) -> Option<EnumMap<DiceColor, i32>> {
    let mut dice_limits = EnumMap::default();
    for game_set in game_line.split(';') {
        for dice_capture in DICE_COUNT_REGEX.captures_iter(game_set) {
            let dice_count: i32 = dice_capture.get(1)?.as_str().parse().ok()?;
            let dice_color = get_dice_color_from_string(dice_capture.get(2)?.as_str())?;
            if dice_count > dice_limits[dice_color] {
                dice_limits[dice_color] = dice_count;
            }
        }
    }
    Some(dice_limits)
}
