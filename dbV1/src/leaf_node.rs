use crate::{constants::*, cursor::Cursor, internal_node::InternalNode, node::{Node, NodeType}, row::Row};


pub struct LeafNode{
    
}

impl LeafNode{
    pub fn leaf_node_num_cells(node: &mut Vec<u8>,) -> &mut i32 {
        let num_cells_bytes = &mut node[LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + 4];
        unsafe { &mut *(num_cells_bytes.as_mut_ptr() as *mut i32) }
    }

    pub fn leaf_node_cell(node: &mut Vec<u8>, cell_num: usize) -> &mut [u8] {
        let start =  LEAF_NODE_HEADER_SIZE.checked_add(
            cell_num.checked_mul(
                LEAF_NODE_CELL_SIZE
            ).expect("> attempt to multiply with overflow")
        ).expect("> attempt to add with overflow");

        let end = start.checked_add(LEAF_NODE_CELL_SIZE).expect("> attempt to add with overflow");
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
        let next_leaf_page_number = Self::leaf_node_next_leaf(node);
        *next_leaf_page_number = 0;
        Node::set_node_type(node, NodeType::NodeLeaf);
        Node::set_node_root(node, false);
    }

    pub fn leaf_node_next_leaf(node: &mut Vec<u8>) -> &mut i32{
        unsafe  {
            &mut *(node[LEAF_NODE_NEXT_LEAF_OFFSET..(LEAF_NODE_NEXT_LEAF_OFFSET + LEAF_NODE_NEXT_LEAF_SIZE)].as_mut_ptr() as *mut i32)
        }  
    }

    pub fn leaf_node_insert(cursor: &mut Cursor, key: i32, value: & Row) {
        let node = cursor.table.pager.get_page(cursor.page_num);
        let num_cells = *LeafNode::leaf_node_num_cells(node);

        let mut page_full: Vec<u8> = vec![0u8; PAGE_SIZE];

        page_full.copy_from_slice(&node[0..PAGE_SIZE]);

        if num_cells as usize >= LEAF_NODE_MAX_CELLS {
            Self::leaf_node_split_and_insert(cursor, key, value);
            return;
        }

        if cursor.cell_num < num_cells {
            for i in (cursor.cell_num..num_cells).rev() {
                let dest = Self::leaf_node_cell(node, i as usize + 1);
                let src = Self::leaf_node_cell(&mut page_full, i as usize);
                dest.copy_from_slice(src);
            }
        }

        let key_ptr = LeafNode::leaf_node_key(node, cursor.cell_num);
        *key_ptr = key;
        Row::serialize_row(value, LeafNode::leaf_node_value(node, cursor.cell_num));

        let num_cells = LeafNode::leaf_node_num_cells(node);
        *num_cells += 1;

    }

    pub fn leaf_node_split_and_insert(cursor: &mut Cursor, _key: i32, value: &Row) {
        let new_page_num: usize = cursor.table.pager.get_unused_page();

        // Node::set_node_root(cursor.table.pager.get_page(cursor.page_num), false);

        let mut copy_of_initial_vector: Vec<u8> = cursor.table.pager.get_page(cursor.page_num).clone();
        let is_old_root:bool = Node::is_node_root(&mut copy_of_initial_vector);                        
        
        let old_max = Node::get_node_max_key(&mut copy_of_initial_vector);
        {
            let new_node: &mut Vec<u8> = cursor.table.pager.get_page(new_page_num);
            Self::initialize_leaf_node(new_node);
            *Self::leaf_node_next_leaf(new_node) = *Self::leaf_node_next_leaf(&mut copy_of_initial_vector);
            *Node::node_parent(new_node) = *Node::node_parent(&mut copy_of_initial_vector);
            
            let destin_node = new_node;
            for i in (LEAF_NODE_LEFT_SPLIT_COUNT..(1 + LEAF_NODE_MAX_CELLS)).rev() {
                let index_within_node = i % LEAF_NODE_LEFT_SPLIT_COUNT;
                let destination = Self::leaf_node_cell(destin_node, index_within_node);
                if i as i32 == cursor.cell_num {
                    Row::serialize_row(value, destination);
                } else if  i > cursor.cell_num as usize {
                    unsafe { 
                        std::ptr::copy_nonoverlapping(
                            Self::leaf_node_cell(&mut copy_of_initial_vector, i-1).as_mut_ptr(), 
                            destination.as_mut_ptr(),
                            LEAF_NODE_CELL_SIZE) 
                        };
                    }
                    else {
                    unsafe { 
                        std::ptr::copy_nonoverlapping(
                            Self::leaf_node_cell(&mut copy_of_initial_vector, i).as_mut_ptr(), 
                            destination.as_mut_ptr(),
                            LEAF_NODE_CELL_SIZE) 
                    };
                }
            }
            *(Self::leaf_node_num_cells(destin_node)) = LEAF_NODE_RIGHT_SPLIT_COUNT as i32;
        }
        
        
        {
            let old_node: &mut Vec<u8> = cursor.table.pager.get_page(cursor.page_num);
            *Self::leaf_node_next_leaf(old_node) = new_page_num as i32;
            let destin_node = old_node;
            for i in (0..LEAF_NODE_LEFT_SPLIT_COUNT).rev() {
                let index_within_node = i % LEAF_NODE_LEFT_SPLIT_COUNT;
                let destination = Self::leaf_node_cell(destin_node, index_within_node);
                if i as i32 == cursor.cell_num {
                    Row::serialize_row(value, destination);
                } else if i > cursor.cell_num as usize{
                    unsafe { 
                        std::ptr::copy_nonoverlapping(
                            Self::leaf_node_cell(&mut copy_of_initial_vector, i-1).as_mut_ptr(), 
                            destination.as_mut_ptr(),
                            LEAF_NODE_CELL_SIZE) 
                    };
                } else {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            Self::leaf_node_cell(&mut copy_of_initial_vector, i).as_mut_ptr(), 
                            destination.as_mut_ptr(),
                            LEAF_NODE_CELL_SIZE) 
                    };
                }
            }
            *(Self::leaf_node_num_cells(destin_node)) = LEAF_NODE_LEFT_SPLIT_COUNT as i32;
        }

        if is_old_root {
            // ===> IMPLEMENT create_new_root <===
            let _type = Node::get_node_type(&copy_of_initial_vector);
            match _type {
                NodeType::NodeInternal =>  println!("is_old_root, {} {} Internal", InternalNode::internal_node_num_keys(&mut copy_of_initial_vector), INTERNAL_NODE_MAX_KEYS),
                NodeType::NodeLeaf => println!("is_old_root, {} {} Leaf", LeafNode::leaf_node_num_cells(&mut copy_of_initial_vector), INTERNAL_NODE_MAX_KEYS),
            }
            return Node::create_new_root(cursor, new_page_num);
        } else {
            let parent_page_num = *Node::node_parent(&mut copy_of_initial_vector) as usize;
            let new_max = Node::get_node_max_key(&mut copy_of_initial_vector);

            let parent = cursor.table.pager.get_page(parent_page_num);
            InternalNode::update_internal_node_key(parent, old_max, new_max);
            InternalNode::internal_node_insert(cursor, parent_page_num, new_page_num);
            // std::process::exit(1);
        }
    }

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
        // println!("Leaf node index: {min_index}");
        cursor.cell_num = min_index;
    }    
}