use crate::constants::*;
use crate::cursor::Cursor;
use crate::leaf_node::LeafNode;
use crate::node::{Node, NodeType};

pub struct InternalNode{
    
}

impl InternalNode {

    pub fn internal_node_num_keys(node: &mut Vec<u8>) -> &mut i32{
        let num_cells_bytes = &mut node[INTERNAL_NODE_NUM_KEYS_OFFSET..INTERNAL_NODE_NUM_KEYS_OFFSET + INTERNAL_NODE_NUM_KEYS_SIZE];
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

    pub fn internal_node_cell_reference(page: &mut Vec<u8>, cell_num: i32) -> &mut i32{
        let start: usize = INTERNAL_NODE_HEADER_SIZE + cell_num as usize * INTERNAL_NODE_CELL_SIZE;
        let end = start + INTERNAL_NODE_KEY_SIZE;
        let num_cell_bytes = &mut page[start..end];
        unsafe { &mut *(num_cell_bytes.as_mut_ptr() as *mut i32) }        
    }

    pub fn internal_node_cell_value(page: &Vec<u8>, cell_num: i32) -> i32 {
        let start: usize = INTERNAL_NODE_HEADER_SIZE + cell_num as usize * INTERNAL_NODE_CELL_SIZE;
        let end = start + INTERNAL_NODE_KEY_SIZE;
        let num_cell_bytes = &page[start..end];
        i32::from_ne_bytes(num_cell_bytes.try_into().unwrap())
    }

    pub fn internal_node_key(node: &mut Vec<u8>, cell_num: i32) -> &mut i32{
        let start = INTERNAL_NODE_HEADER_SIZE + cell_num as usize * INTERNAL_NODE_CELL_SIZE + INTERNAL_NODE_CHILD_SIZE;
        let end = start + INTERNAL_NODE_KEY_SIZE;
        let mut temp = node.clone();
        let num_cell_bytes = &mut node[start..end];
        unsafe {
            println!("internal_node_key {start} {end} {cell_num} {}", Self::internal_node_num_keys(&mut temp));
            let val = &mut *(num_cell_bytes.as_mut_ptr() as *mut i32);
            println!("internal_node_key cell_num: {cell_num}, Val: {}", *val);
            return val;
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
            return Self::internal_node_cell_reference(root, child_num);
        }
    }

    pub fn update_internal_node_key(node: &mut Vec<u8>, old_max: i32, new_max: i32) {
        let old_child_index = Self::internal_node_find_child(node, old_max);
        *InternalNode::internal_node_key(node, old_child_index) = new_max;
    }

    pub fn internal_node_insert(cursor: &mut Cursor, parent_page_num: usize, child_page_num: usize) {
        let mut parent: Vec<u8> = cursor.table.pager.get_page_at(parent_page_num);
        let child_max_key: i32;
        {
            let child: &mut Vec<u8> = cursor.table.pager.get_page(child_page_num);
            child_max_key = *Node::get_node_max_key(child);
        }
        let index: i32 = InternalNode::internal_node_find_child(&mut parent, child_max_key);

        let original_num_keys: i32 = *InternalNode::internal_node_num_keys(&mut parent);
        *InternalNode::internal_node_num_keys(&mut parent) = original_num_keys + 1;

        if original_num_keys as usize >= INTERNAL_NODE_MAX_KEYS{
            println!("Need to implement splitting internal node");
            std::process::exit(1);
        }

        let right_child_page_num: i32 = *InternalNode::internal_node_right_child(&mut parent);
        let right_child: &mut Vec<u8> = cursor.table.pager.get_page(right_child_page_num as usize);

        if child_max_key > *Node::get_node_max_key(right_child) {
            *InternalNode::internal_node_child(&mut parent, original_num_keys) = right_child_page_num;
            let temp = *Node::get_node_max_key(right_child);
            println!("------------------------------------ {original_num_keys} {temp} {}", parent.len());
            let a = InternalNode::internal_node_key(&mut parent, original_num_keys);
            println!("======================================");
            *a = temp;
            println!("======================================");
            *InternalNode::internal_node_right_child(&mut parent) = child_page_num as i32;
        } else {
            for i in (1+index..original_num_keys+1).rev() {
                let source = InternalNode::internal_node_cell_value(&parent, i-1);
                let destination = InternalNode::internal_node_cell_reference(&mut parent, i);

                unsafe { 
                    std::ptr::copy_nonoverlapping(
                        &source,
                        destination,
                        INTERNAL_NODE_CELL_SIZE) 
                };                
            }
            *InternalNode::internal_node_child(&mut parent, index) = child_page_num as i32;
            *InternalNode::internal_node_key(&mut parent, index) = child_max_key;
        }
        // copy parent to location
        

        unsafe { 
            std::ptr::copy(
                &parent,
                cursor.table.pager.get_page(parent_page_num),
                parent.len()) 
        };            
    }

    pub fn internal_node_find_child(node: &mut Vec<u8>, key: i32) -> i32{
        println!("internal_node_find_child");
        let num_keys = *InternalNode::internal_node_num_keys(node);
        let mut min_index = 0;
        let mut max_index = num_keys;

        while min_index != max_index {
            let index = min_index + (max_index - min_index) / 2;
            let key_to_right = *InternalNode::internal_node_key(node, index);
            if key_to_right >= key {
                max_index = index;
            } else {
                min_index = index +1;
            }
        }
        println!("Internal node index: {min_index}");
        return min_index
    }

    pub fn internal_node_find(cursor: &mut Cursor, page_num: usize, key: i32){
        let node = cursor.table.pager.get_page(page_num);
        let child_index = Self::internal_node_find_child(node, key);
        let child_num = *Self::internal_node_child(node, child_index) as usize;
        let child = cursor.table.pager.get_page(child_num);
        cursor.page_num = child_num;
        match Node::get_node_type(&child) {
            NodeType::NodeInternal => {
                println!("Child is internal node");
                return Self::internal_node_find(cursor, child_num, key);
            },
            NodeType::NodeLeaf => {
                println!("Child is leaf node");
                return LeafNode::leaf_node_search(cursor, child_num, key);
            },
        }
    }
}

