use clio::Input;
use pipe_map::PipeTileMap;

mod pipe_map;
mod pipe_tile;

pub fn loop_length_in_file(path: std::ffi::OsString) -> u64 {
    loop_length_in_input(clio::Input::new(&path).unwrap())
}

fn loop_length_in_input(input: Input) -> u64 {
    let map = get_pipe_map_from_input(input);
    map.get_pipe_loop_coords_from_start().len().try_into().unwrap()
}

pub fn get_pipe_map_from_input(input: Input) -> PipeTileMap {
    let input = std::io::BufReader::new(input);
    PipeTileMap::from_iter(std::io::BufRead::lines(input).map(|line| line.unwrap())).unwrap()
}

pub fn enclosed_tiles_in_file(path: std::ffi::OsString) -> u64 {
    enclosed_tiles_in_input(clio::Input::new(&path).unwrap())
}

pub fn enclosed_tiles_in_input(input:Input)->u64{
    let map = get_pipe_map_from_input(input);
    map.get_enclosed_tile_count()
}
