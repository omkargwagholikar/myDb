use crate::constants::*;
use crate::cursor::Cursor;
use crate::leaf_node::LeafNode;
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
    
    pub fn internal_node_right_child(root: &mut Vec<u8>) -> &mut i32{
        let num_cell_bytes = &mut root[INTERNAL_NODE_RIGHT_CHILD_OFFSET..(INTERNAL_NODE_RIGHT_CHILD_OFFSET + INTERNAL_NODE_RIGHT_CHILD_SIZE)];
        unsafe { &mut *(num_cell_bytes.as_mut_ptr() as *mut i32) }
    }

    pub fn internal_node_cell(page: &mut Vec<u8>, cell_num: i32) -> &mut i32{
        let start: usize = INTERNAL_NODE_HEADER_SIZE + cell_num as usize * INTERNAL_NODE_CELL_SIZE;
        let end = start + INTERNAL_NODE_KEY_SIZE;
        let num_cell_bytes = &mut page[start..end];
        unsafe { &mut *(num_cell_bytes.as_mut_ptr() as *mut i32) }        
    }

    // pub fn internal_node_key(node: &mut Vec<u8>, key_num: i32) -> &mut i32 {
    //     let start: usize = INTERNAL_NODE_HEADER_SIZE + key_num as usize * INTERNAL_NODE_CELL_SIZE;
    //     let end = start + INTERNAL_NODE_KEY_SIZE;
    //     let num_cell_bytes = &mut node[start..end];
    //     unsafe { &mut *(num_cell_bytes.as_mut_ptr() as *mut i32) }
    // }

    pub fn internal_node_key(root: &mut Vec<u8>, key_num: i32) -> &mut i32{
        let int_node_cell = Self::internal_node_cell(root, key_num) as *mut i32;
        unsafe  {
            &mut *int_node_cell.add(INTERNAL_NODE_CHILD_SIZE)
        }        
    }

    pub fn internal_node_child(root: &mut Vec<u8>, child_num: i32) -> &mut i32{
        let num_keys = *Self::internal_node_num_keys(root);
        if child_num > num_keys {
            println!("Trying to access child_num {child_num} > num_keys {num_keys}");
            std::process::exit(1);
        } else if child_num == num_keys {
            return  Self::internal_node_right_child(root);
        } else {
            return Self::internal_node_cell(root, child_num);
        }
    }

    pub fn internal_node_find(cursor: &mut Cursor, page_num: usize, key: i32){
        let node = cursor.table.pager.get_page(page_num);
        let num_keys = *Self::internal_node_num_keys(node);

        let mut low = 0;
        let mut high= num_keys;

        while low != high {
            let index = low + (high - low) / 2;
            let key_to_right = *Self::internal_node_key(node, index);
            if key_to_right >= key {
                high = index;
            } else {
                low = index + 1;
            }
        }
        let child_num = *Self::internal_node_child(node, low) as usize;
        let child = cursor.table.pager.get_page(child_num);
        match Node::get_node_type(&child) {
            NodeType::NodeInternal => {
                return Self::internal_node_find(cursor, child_num, key);
            },
            NodeType::NodeLeaf => {
                return LeafNode::leaf_node_search(cursor, child_num, key);
            },
        }
    }
}

