use crate::constants::*;
use crate::pager::Pager;

pub struct Table {
    pub num_rows: usize,
    pub pager: Pager
}

impl Table {
    pub fn new(file_name: &str) -> Table{
        let temp_pager = Pager::new(file_name);
        let temp_num_rows = temp_pager.file_length / ROW_SIZE;
        Table {
            pager: temp_pager,
            num_rows: temp_num_rows,
        }
    }
    
    pub fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
        let page = self.pager.get_page(page_num);
        &mut page[byte_offset..byte_offset + ROW_SIZE]
    }

    pub fn db_close(&mut self) {
        let num_full_pages = self.num_rows / ROWS_PER_PAGE;
        
        for i in 0..num_full_pages {
            if self.pager.pages[i].is_none() {
                continue;
            } else {
                self.pager.flush(i, PAGE_SIZE);
                self.pager.pages[i] = None;
            }
        }

        let num_additional_rows = self.num_rows % ROWS_PER_PAGE;
        if num_additional_rows > 0 {
            let page_num = num_full_pages;
            if !self.pager.pages[page_num].is_none() {
                self.pager.flush(page_num, num_additional_rows * ROW_SIZE);
                self.pager.pages[page_num] = None;
            }
        }
    }
}
