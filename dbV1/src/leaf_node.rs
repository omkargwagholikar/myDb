use crate::{constants::*, cursor::Cursor, row::Row,};

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
        let start_value = start + LEAF_NODE_KEY_OFFSET;
        let end_value = start + LEAF_NODE_KEY_SIZE;
        let key_byte = &mut node[start_value..end_value];
        unsafe { 
            return  &mut *(key_byte.as_mut_ptr() as *mut i32);
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
            println!("Split node required (not implemented)");
            std::process::exit(1); 
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

    pub fn leaf_node_search(cursor: &mut Cursor, page_num: usize, key: i32) {
        let page = cursor.table.pager.get_page(page_num);
        let num_cells = *Self::leaf_node_num_cells(page);
        let mut min_index = 0;
        let mut high_index = num_cells - 1;    
        
        while min_index <= high_index {
            let mid_index = min_index + (high_index - min_index) / 2;
    
            let key_at_index = *Self::leaf_node_key(page, mid_index);
            let mut row = Row::new();
            Row::deserialize_row(&Self::leaf_node_cell(page, mid_index), &mut row);
            println!("{} {}", key_at_index, row.id);
            // println!("{} {} {} {}", high_index, mid_index, min_index, key_at_index);
    
            if key == row.id as i32 {
                cursor.cell_num = mid_index;
                return;
            } else if key < row.id as i32 {
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