use reflection_grid::ReflectionGrid;

mod reflection_grid;

pub fn find_all_reflections_in_file(path: std::ffi::OsString)->impl Iterator<Item=ReflectionResult>{
    find_all_reflections_in_input(clio::Input::new(&path).unwrap())
}

pub fn find_all_alternate_reflections_in_file(path:std::ffi::OsString)->impl Iterator<Item=ReflectionResult>{
    find_all_alternate_reflections_in_input(clio::Input::new(&path).unwrap())
}

fn find_all_alternate_reflections_in_input(input: clio::Input)->impl Iterator<Item=ReflectionResult>{{
    let reflection_patterns = get_all_reflections_from_input(input);
    reflection_patterns.into_iter().filter_map(|grid|grid.get_alternate_reflection_result())
}}

fn find_all_reflections_in_input(input: clio::Input)->impl Iterator<Item=ReflectionResult>{
    let reflection_patterns = get_all_reflections_from_input(input);
    reflection_patterns.into_iter().filter_map(|grid|grid.get_reflection_result())
}

fn get_all_reflections_from_input(input: clio::Input)->Vec<ReflectionGrid>{
    let mut reflection_grids = Vec::new();
    let mut line_iter = std::io::BufRead::lines(std::io::BufReader::new(input)).map_while(Result::ok);
    while let Some(reflection_grid) = ReflectionGrid::from_iter(&mut line_iter){
        reflection_grids.push(reflection_grid);
    }
    reflection_grids
}

pub struct ReflectionResult{
    pub reflection_type:ReflectionType,
    pub lines_to_left_or_above_reflection:u64,
}

pub enum ReflectionType{
    VerticalLine,
    HorizontalLine,
}