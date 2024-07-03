use crate::week2::node_network;

#[test]
fn day8_1_test1() {
    assert_eq!(
        2,
        node_network::steps_to_navigate_network_in_file("inputs/week2/input_081_test".into())
    );
}

#[test]
fn day8_1_test2() {
    assert_eq!(
        16897,
        node_network::steps_to_navigate_network_in_file("inputs/week2/input_08".into())
    );
}

#[test]
fn day8_2_test1() {
    assert_eq!(
        6,
        node_network::steps_to_navigate_network_in_file_simul("inputs/week2/input_082_test".into())
    );
}

#[test]
fn day8_2_test2() {
    assert_eq!(
        16563603485021,
        node_network::steps_to_navigate_network_in_file_simul("inputs/week2/input_08".into())
    );
}
