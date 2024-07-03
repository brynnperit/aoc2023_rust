use std::{cell::RefCell, rc::Weak};

use crate::week2::node_network::network_instruction::NetworkInstruction;

pub struct NetworkNode {
    name: String,
    left: Option<Weak<RefCell<NetworkNode>>>,
    right: Option<Weak<RefCell<NetworkNode>>>,
}

impl NetworkNode {
    pub fn from_string(name: String) -> Self {
        NetworkNode {
            name,
            left: None,
            right: None,
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn set_linked(
        &mut self,
        link_direction: NetworkInstruction,
        node_to_link: Weak<RefCell<NetworkNode>>,
    ) {
        match link_direction {
            NetworkInstruction::L => self.left = Some(node_to_link),
            NetworkInstruction::R => self.right = Some(node_to_link),
        }
    }

    pub fn get_linked(&self, link_direction: NetworkInstruction) -> Weak<RefCell<NetworkNode>> {
        match link_direction {
            NetworkInstruction::L => self.left.as_ref().unwrap().clone(),
            NetworkInstruction::R => self.right.as_ref().unwrap().clone(),
        }
    }
}
