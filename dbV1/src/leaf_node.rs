use crate::{constants::*, cursor::Cursor, pager::Pager, row::Row, table};

#[derive(PartialEq)]
pub enum NodeType {
    NodeInternal,
    NodeLeaf
}

pub struct LeafNode{
    
}

impl LeafNode{
    pub fn leaf_node_num_cells(node: &mut Vec<u8>,) -> &mut i32 {
        let num_cells_bytes = &mut node[LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + 4];
        unsafe { &mut *(num_cells_bytes.as_mut_ptr() as *mut i32) }
    }

    pub fn leaf_node_cell(node: &mut Vec<u8>, cell_num: i32) -> &mut [u8] {
        let start = LEAF_NODE_HEADER_SIZE + (cell_num as usize) * LEAF_NODE_CELL_SIZE;
        let end = start + LEAF_NODE_CELL_SIZE;
        &mut node[start..end]
    }

    pub fn leaf_node_key(node: &mut Vec<u8>, cell_num: i32) -> &mut i32 {
        let start = LEAF_NODE_HEADER_SIZE + (cell_num as usize) * LEAF_NODE_CELL_SIZE;
        let key_offset = start + LEAF_NODE_KEY_OFFSET;
        let key_bytes = &mut node[key_offset..key_offset + LEAF_NODE_KEY_SIZE];
        unsafe { 
            &mut *(key_bytes.as_mut_ptr() as *mut i32)
        }
    }
    

    pub fn leaf_node_value(node: &mut Vec<u8>, cell_num: i32) -> &mut [u8] {
        let start_node = LEAF_NODE_HEADER_SIZE + (cell_num as usize) * LEAF_NODE_CELL_SIZE;
        let start_value = start_node + LEAF_NODE_VALUE_OFFSET;
        let end_node = start_value + LEAF_NODE_VALUE_SIZE;
        return &mut node[start_node..end_node];
    }

    pub fn initialize_leaf_node(node: &mut Vec<u8>) {
        let num_cells = Self::leaf_node_num_cells(node);
        *num_cells = 0;
        Self::set_node_type(node, NodeType::NodeLeaf);

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

    pub fn leaf_node_insert(cursor: &mut Cursor, key: i32, value: & Row) {
        let node = cursor.table.pager.get_page(cursor.page_num);
        let num_cells = *LeafNode::leaf_node_num_cells(node);

        let mut page_full: Vec<u8> = vec![0u8; PAGE_SIZE];

        page_full.copy_from_slice(&node[0..PAGE_SIZE]);

        if num_cells as usize >= LEAF_NODE_MAX_CELLS {
            // println!("Split node required (not implemented)");
            // std::process::exit(1); 
            Self::leaf_node_split_and_insert(cursor, key, value);
            return;
        }

        if cursor.cell_num < num_cells {
            for i in (cursor.cell_num..num_cells).rev() {
                let dest = Self::leaf_node_cell(node, i + 1);
                let src = Self::leaf_node_cell(&mut page_full, i);
                dest.copy_from_slice(src);
            }
        }

        let key_ptr = LeafNode::leaf_node_key(node, cursor.cell_num);
        *key_ptr = key;
        Row::serialize_row(value, LeafNode::leaf_node_value(node, cursor.cell_num));

        let num_cells = LeafNode::leaf_node_num_cells(node);
        *num_cells += 1;

    }

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

    pub fn leaf_node_split_and_insert(cursor: &mut Cursor, _key: i32, value: &Row) {
        // ===> IMPLEMENT get_unused_page <===
        let new_page_num: usize = cursor.table.pager.get_unused_page();
        // let new_node: &mut Vec<u8> = cursor.table.pager.get_page(new_page_num);
        let mut copy_of_initial_vector: Vec<u8> = cursor.table.pager.get_page(cursor.page_num).clone();
        let is_old_root:bool = Self::is_node_root(&mut copy_of_initial_vector);

        {
            let new_node: &mut Vec<u8> = cursor.table.pager.get_page(new_page_num);
            Self::initialize_leaf_node(new_node);
            let destin_node = new_node;
            for i in (LEAF_NODE_LEFT_SPLIT_COUNT..(1 + LEAF_NODE_MAX_CELLS)).rev() {
                let index_within_node = i % LEAF_NODE_LEFT_SPLIT_COUNT;
                let destination = Self::leaf_node_cell(destin_node, index_within_node as i32);
                if i as i32 == cursor.cell_num {
                    Row::serialize_row(value, destination);
                } else {
                    unsafe { 
                        std::ptr::copy_nonoverlapping(
                            Self::leaf_node_cell(&mut copy_of_initial_vector, (i as i32)-1).as_mut_ptr(), 
                            destination.as_mut_ptr(),
                            LEAF_NODE_CELL_SIZE) 
                    };
                }
            }
            *(Self::leaf_node_num_cells(destin_node)) = LEAF_NODE_RIGHT_SPLIT_COUNT as i32;
        }
        
        
        {
            let old_node: &mut Vec<u8> = cursor.table.pager.get_page(cursor.page_num);
            let destin_node = old_node;
            for i in (0..LEAF_NODE_LEFT_SPLIT_COUNT).rev() {
                let index_within_node = i % LEAF_NODE_LEFT_SPLIT_COUNT;
                let destination = Self::leaf_node_cell(destin_node, index_within_node as i32);
                if i as i32 == cursor.cell_num {
                    Row::serialize_row(value, destination);
                } else {
                    unsafe { 
                        std::ptr::copy_nonoverlapping(
                            Self::leaf_node_cell(&mut copy_of_initial_vector, (i as i32)-1).as_mut_ptr(), 
                            destination.as_mut_ptr(),
                            LEAF_NODE_CELL_SIZE) 
                    };
                }
            }
            *(Self::leaf_node_num_cells(destin_node)) = LEAF_NODE_LEFT_SPLIT_COUNT as i32;
        }

        if is_old_root {
            // ===> IMPLEMENT create_new_root <===
            // return Self::create_new_root(cursor, new_page_num);
        } else {
            println!("Need to implement updating parent after split");
            std::process::exit(1);
        }
    }

    // pub fn create_new_root(cursor: &mut Cursor, right_child_page_num: usize) {
    //     let left_child_page_num = cursor.table.pager.num_pages;
    //     let left_child = cursor.table.pager.get_page(left_child_page_num);

    //     let root = cursor.table.pager.get_page(cursor.table.root_page_num);
    //     let right_child = cursor.table.pager.get_page(right_child_page_num);
        
    //     unsafe { std::ptr::copy_nonoverlapping(left_child.as_mut_ptr(), root.as_mut_ptr(), PAGE_SIZE);}
    // }

    pub fn leaf_node_search(cursor: &mut Cursor, page_num: usize, key: i32) {

        let page = cursor.table.pager.get_page(page_num);
        let num_cells = *Self::leaf_node_num_cells(page);
        let mut min_index = 0;
        let mut high_index = num_cells - 1;    
        
        while min_index <= high_index {
            let mid_index = min_index + (high_index - min_index) / 2;
    
            let key_at_index = *Self::leaf_node_key(page, mid_index);
    
            if key == key_at_index {
                cursor.cell_num = mid_index;
                return;
            } else if key < key_at_index {
                if mid_index == 0 {
                    break; 
                }
                high_index = mid_index - 1;
            } else {
                min_index = mid_index + 1;
            }
        }
            
        cursor.cell_num = min_index;
    }    
}