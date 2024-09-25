use crate::constants::*;


#[derive(PartialEq)]
pub enum NodeType {
    NodeInternal,
    NodeLeaf
}

pub struct  Node {

}

impl Node {
    pub fn is_node_root(node: &Vec<u8>) -> bool{
        let val = &node[0..IS_ROOT_OFFSET];
        if val[0] > 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn set_node_root(page: &mut Vec<u8>, is_root: bool) {
        // This will only work if the size of is_root is one byte only
        page[IS_ROOT_OFFSET] = is_root as u8;
    }

    pub fn get_node_type(node: & Vec<u8>) -> NodeType{
        let value: u8 = node[NODE_TYPE_OFFSET];
        return match value {
            0 => NodeType::NodeInternal,
            1 => NodeType::NodeLeaf,
            _ => panic!("Invalid node type!"), // Handle unexpected values
        }
    }

    pub fn set_node_type(node: &mut Vec<u8>, node_type: NodeType) {
        let value:u8 = match node_type {
            NodeType::NodeInternal => 0,
            NodeType::NodeLeaf => 1,
        };
        node[NODE_TYPE_OFFSET] = value;
    }
}