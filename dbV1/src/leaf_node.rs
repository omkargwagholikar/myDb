use crate::{constants::*, table::Table, row::Row, cursor::Cursor};

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
        let key_bytes = &mut node[key_offset..key_offset + 4];
        unsafe { &mut *(key_bytes.as_mut_ptr() as *mut i32) }
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
    }

    // pub fn leaf_node_insert(cursor: &mut Cursor, key: i32, value: &mut Row) {
    //     let node = cursor.table.pager.get_page(cursor.page_num);
    //     let num_cells = *LeafNode::leaf_node_num_cells(node);

    //     // Check if the leaf node is full, and handle the split if needed.
    //     if num_cells as usize >= LEAF_NODE_MAX_CELLS {
    //         println!("Split node required (not implemented)");
    //         std::process::exit(1); 
    //     }

    //     if cursor.cell_num < num_cells {
    //         for i in (cursor.cell_num..num_cells).rev() {
    //             let dest = Self::leaf_node_cell(node, i + 1);
    //             let src = Self::leaf_node_cell(node, i);
    //             dest.copy_from_slice(src); // Move the cell data one position forward
    //         }
    //     }

    //     let cell = LeafNode::leaf_node_cell(node, cursor.cell_num);
    //     let key_ptr = LeafNode::leaf_node_key(node, cursor.cell_num);
    //     *key_ptr = key;

    //     Row::serialize_row(value, LeafNode::leaf_node_value(node, cursor.cell_num));

    //     let num_cells = LeafNode::leaf_node_num_cells(node);
    //     *num_cells += 1;
    // }
}