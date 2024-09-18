use crate::constants::*;

pub struct Table {
    pub num_rows: usize,
    pub pages: [Option<Vec<u8>>; TABLE_MAX_PAGES], 
}

impl Table {
    pub fn new() -> Table{
        Table {
            num_rows: 0,
            pages: std::array::from_fn(|_| None)
        }
    }
    pub fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        if self.pages[page_num].is_none() {
            self.pages[page_num] = Some(vec![0; PAGE_SIZE]);
        }

        let page = self.pages[page_num].as_mut().unwrap();
        &mut page[byte_offset..(byte_offset + ROW_SIZE)]
    }

    pub fn drop_table(&mut self) {
        for page in self.pages.iter_mut() {
            if let Some(_) = page {
                *page = None
            }
        }
        self.num_rows = 0;
    }
}
