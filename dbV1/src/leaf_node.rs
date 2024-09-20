use crate::constants::*;


struct LeafNode {
}

impl LeafNode {
    pub fn leaf_node_num_cells(node: &mut [u8]) -> i32 {
        let num_cells_bytes = &node[LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + 4];
        i32::from_le_bytes(num_cells_bytes.try_into().unwrap())
    }

    // pub fn leaf_node_cell(node: &mut [u8] , cell_num: i32) -> &mut [u8] {
    //     return node + LEAF_NODE_HEADER_SIZE + cell_num * LEAF_NODE_CELL_SIZE;
    //     return None;
    // }

}