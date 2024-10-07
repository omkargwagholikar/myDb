use std::io::Write;

use crate::{constants::*, internal_node::InternalNode, leaf_node::LeafNode, pager::Pager};


#[derive(PartialEq)]
pub enum NodeType {
    NodeInternal,
    NodeLeaf
}

pub struct  Node {

}

impl Node {
    pub fn is_node_root(node: &Vec<u8>) -> bool{
        let val = &node[IS_ROOT_OFFSET];
        if *val > 0 {
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

    pub fn get_node_max_key(node: &mut Vec<u8>) -> &mut i32{
        let mut temp = node.clone();
        match Self::get_node_type(&temp) {
            NodeType::NodeInternal => {
                return InternalNode::internal_node_key(node, *InternalNode::internal_node_num_keys(&mut temp)-1);
            },
            NodeType::NodeLeaf => {
                return LeafNode::leaf_node_key(node, *LeafNode::leaf_node_num_cells(&mut temp) - 1);
            }
        }
    }

    pub fn node_parent(node: &mut Vec<u8>) -> &mut i32 {
        let num_cells_bytes = &mut node[PARENT_POINTER_OFFSET..PARENT_POINTER_OFFSET + PARENT_POINTER_SIZE];
        unsafe { &mut *(num_cells_bytes.as_mut_ptr() as *mut i32) }        
    }

    pub fn indent(level: i32) {
        for _ in 0..level {
            print!(" ");
        }
        std::io::stdout().flush().unwrap();
    }

    pub fn print_tree(pager: &mut Pager, page_num: usize, indent_level: i32) {
        let mut node = pager.get_page_at(page_num);
    
        match Self::get_node_type(&node) {
            NodeType::NodeInternal => {
                let num_keys = *InternalNode::internal_node_num_keys(&mut node);
                Self::indent(indent_level);
                println!("- internal size: {} {}", num_keys, Node::is_node_root(&node));
                for i in 0..num_keys {
                    let child = *InternalNode::internal_node_child(&mut node, i);
                    Self::print_tree(pager, child as usize, indent_level + 1);
    
                    Self::indent(indent_level + 1);
                    println!("- key {}", InternalNode::internal_node_key(&mut node, i));
                }
                let child = *InternalNode::internal_node_right_child(&mut node);
                Self::print_tree(pager, child as usize, indent_level + 1);
            },
            NodeType::NodeLeaf => {
                let num_keys = *LeafNode::leaf_node_num_cells(&mut node);
                Self::indent(indent_level);
                println!("- leaf size: {} {}", num_keys, Node::is_node_root(&node));
                for i in 0..num_keys {
                    Self::indent(indent_level + 1);
                    println!("- {}", *LeafNode::leaf_node_key(&mut node, i));
                }
            },
        }        
    }
}