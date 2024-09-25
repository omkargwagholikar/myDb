use crate::constants::*;
use crate::node::{Node, NodeType};

pub struct InternalNode{
    
}

impl InternalNode {

    pub fn internal_node_num_keys(node: &mut Vec<u8>) -> &mut i32{
        let num_cells_bytes = &mut node[LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + 4];
        unsafe { &mut *(num_cells_bytes.as_mut_ptr() as *mut i32) }
    }

    pub fn initialize_internal_node(root: &mut Vec<u8>) {
        Node::set_node_type(root, NodeType::NodeInternal);
        Node::set_node_root(root, false);
        *InternalNode::internal_node_num_keys(root) = 0;
    }

    // pub fn internal_node_child(root: &mut Vec<u8>, child_num: i32) -> &mut i32{
    //     let num_keys = *Self::internal_node_num_keys(root);
    //     if child_num > num_keys {
    //         println!("Trying to access child_num {child_num} > num_keys {num_keys}");
    //         std::process::exit(1);
    //     } else if child_num == num_keys {
    //         return  // ==> the right child; <==
    //     } else {
    //         return // ==> the internal node cell <==
    //     }
    // }


    // pub fn internal_node_right_child(root: &mut Vec<u8>) -> &mut i32{
    //
    // }

    

}

