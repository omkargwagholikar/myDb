use crate::constants::*;
use crate::leaf_node::LeafNode;
use crate::pager::Pager;

pub struct Table {
    pub pager: Pager,
    pub root_page_num: usize
}

impl Table {
    pub fn new(file_name: &str) -> Table{
        let mut pager = Pager::new(file_name);
        let root_page_num = 0;
        
        if pager.num_pages == 0 {
            let root_data = pager.get_page(0);
            LeafNode::initialize_leaf_node(root_data);
        }
        
        Table {
            pager,
            root_page_num
        }
    }
    
    // // This funtion is replaced by cursor::cursor_value at that row, 
    // // this helps to create an abstraction as the actual arrangement 
    // // of data within the table will change when implementing the b tree

    // pub fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
    //     let page_num = row_num / ROWS_PER_PAGE;
    //     let row_offset = row_num % ROWS_PER_PAGE;
    //     let byte_offset = row_offset * ROW_SIZE;
    //     let page = self.pager.get_page(page_num);
    //     &mut page[byte_offset..byte_offset + ROW_SIZE]
    // }

    pub fn db_close(&mut self) {
        
        for i in 0..self.pager.num_pages {
            if !self.pager.pages[i].is_none() {
                self.pager.flush(i);
                self.pager.pages[i] = None;
            }
        }

    }
}
