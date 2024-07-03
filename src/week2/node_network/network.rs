use std::{cell::RefCell, collections::HashMap, rc::Rc};

use network_node::NetworkNode;

use super::network_instruction::NetworkInstruction;

mod network_node;
pub struct Network {
    nodes: HashMap<String, Rc<RefCell<NetworkNode>>>,
}

impl Network {
    pub fn from_iterator(str_iter: impl Iterator<Item = String>) -> Self {
        let mut node_names = Vec::new();
        let mut left_linked_node_names = Vec::new();
        let mut right_linked_node_names = Vec::new();
        for line in str_iter {
            let mut line = line.split_ascii_whitespace();
            node_names.push(line.next().unwrap().to_string());
            line.next();
            left_linked_node_names.push(
                line.next()
                    .unwrap()
                    .split_once('(')
                    .unwrap()
                    .1
                    .split_once(',')
                    .unwrap()
                    .0
                    .to_string(),
            );
            right_linked_node_names
                .push(line.next().unwrap().split_once(')').unwrap().0.to_string());
        }
        let mut nodes = HashMap::new();
        for name in node_names.iter() {
            nodes.insert(
                name.clone(),
                Rc::new(RefCell::new(NetworkNode::from_string(name.clone()))),
            );
        }
        for trio in node_names
            .into_iter()
            .zip(left_linked_node_names.into_iter())
            .zip(right_linked_node_names.into_iter())
        {
            let node_name = trio.0 .0;
            let left_node_name = trio.0 .1;
            let right_node_name = trio.1;

            let node = nodes.get(&node_name).unwrap();
            let node_left = nodes.get(&left_node_name).unwrap().to_owned();
            let node_right = nodes.get(&right_node_name).unwrap().to_owned();

            node.as_ref()
                .borrow_mut()
                .set_linked(NetworkInstruction::L, Rc::downgrade(&node_left));
            node.as_ref()
                .borrow_mut()
                .set_linked(NetworkInstruction::R, Rc::downgrade(&node_right));
        }
        Network { nodes }
    }

    pub fn all_node_names<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.nodes.iter().map(|pair| pair.0.as_str())
    }

    pub fn get_node(&self, node_name: &str) -> Rc<RefCell<NetworkNode>> {
        self.nodes.get(node_name).unwrap().clone()
    }
}
