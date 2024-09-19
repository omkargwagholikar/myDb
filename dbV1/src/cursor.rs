use crate::table::Table;
use crate::constants::*;

pub struct Cursor<'a> {
    pub table: &'a mut Table,  // Mutable reference to Table
    pub row_num: usize,
    pub end_of_table: bool,
}

impl<'a> Cursor<'a> {
    pub fn new(table: &'a mut Table) -> Cursor<'a> {
        let end_row = table.num_rows == 0;
        Cursor {
            table: table,
            row_num: 0,
            end_of_table: end_row,
        }
    }

    pub fn table_start(&mut self) {
        self.end_of_table = self.table.num_rows <= 0;
        self.row_num = 0;        
    }

    pub fn table_end(&mut self) {
        self.row_num = self.table.num_rows;
        self.end_of_table = true;
    }

    pub fn advance_cursor(&mut self) {
        self.row_num += 1;
        if self.row_num >= self.table.num_rows { self.end_of_table = true };
    }
    pub fn cursor_value(&mut self) -> &mut [u8] {
        let page_num = self.row_num / ROWS_PER_PAGE;
        let row_offset = self.row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
        let page = self.table.pager.get_page(page_num);
        &mut page[byte_offset..byte_offset + ROW_SIZE]
    }
}