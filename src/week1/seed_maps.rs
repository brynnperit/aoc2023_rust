mod seed_maps_numbers;
mod seed_maps_ranges;
mod source_target_map;

pub fn get_location_numbers_from_seed_file(path: std::ffi::OsString) -> Vec<usize> {
    seed_maps_numbers::get_location_numbers_from_seed_input(clio::Input::new(&path).unwrap())
}

pub fn get_location_numbers_from_seed_file_as_ranges(path: std::ffi::OsString) -> Vec<usize> {
    seed_maps_ranges::get_location_numbers_from_seed_input_as_ranges(clio::Input::new(&path).unwrap())
}
