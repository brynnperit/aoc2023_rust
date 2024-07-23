use crate::week2::reflected_patterns;

fn get_reflection_total(reflection_results: impl Iterator<Item = reflected_patterns::ReflectionResult>)->u64{
    reflection_results.map(|result|{
        match result.reflection_type{
            reflected_patterns::ReflectionType::VerticalLine => result.lines_to_left_or_above_reflection,
            reflected_patterns::ReflectionType::HorizontalLine => result.lines_to_left_or_above_reflection*100,
        }
    }).sum()
}

#[test]
fn day13_1_test1() {
    let reflection_results = reflected_patterns::find_all_reflections_in_file("inputs/week2/input_131_test".into());
    assert_eq!(
        405_u64,
        get_reflection_total(reflection_results),
    )
}

#[test]
fn day13_1_test2() {
    let reflection_results = reflected_patterns::find_all_reflections_in_file("inputs/week2/input_13".into());
    assert_eq!(
        37113_u64,
        get_reflection_total(reflection_results),
    )
}

#[test]
fn day13_2_test1() {
    let reflection_results = reflected_patterns::find_all_alternate_reflections_in_file("inputs/week2/input_131_test".into());
    assert_eq!(
        400_u64,
        get_reflection_total(reflection_results),
    )
}

#[test]
fn day13_2_test2() {
    let reflection_results = reflected_patterns::find_all_alternate_reflections_in_file("inputs/week2/input_13".into());
    assert_eq!(
        30449_u64,
        get_reflection_total(reflection_results),
    )
}
