use crate::constants::*;

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
}
