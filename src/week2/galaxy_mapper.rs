use galaxy_map::GalaxyMap;

use crate::shared_libs::coord_2_d::Coord2D;

mod galaxy_map;

pub fn shortest_distances_between_galaxies_in_file(
    path: std::ffi::OsString,
    expansion_factor: ExpansionFactor,
) -> Vec<(Coord2D<usize>, Coord2D<usize>, u64)> {
    shortest_distances_between_galaxies_in_input(clio::Input::new(&path).unwrap(), expansion_factor)
}

fn shortest_distances_between_galaxies_in_input(
    input: clio::Input,
    expansion_factor: ExpansionFactor,
) -> Vec<(Coord2D<usize>, Coord2D<usize>, u64)> {
    let map = galaxy_map_from_input(input);
    map.distances_between_all_pairs(expansion_factor)
}

fn galaxy_map_from_input(input: clio::Input) -> GalaxyMap {
    let input = std::io::BufReader::new(input);
    GalaxyMap::from_input(std::io::BufRead::lines(input).map(|line| line.unwrap()))
}

pub enum ExpansionFactor {
    Multiply(u64),
}
