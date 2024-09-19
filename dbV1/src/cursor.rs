use crate::table::Table;


pub struct Cursor {
    pub table: Table,
    pub row_num: usize,
    pub end_of_table: bool
}

impl Cursor {
    pub fn new(file_name: &str) -> Cursor{
        let table = Table::new(file_name);
        let end_row = table.num_rows == 0;
        Cursor {
            table: table,
            row_num: 0,
            end_of_table:end_row
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
        self.table.row_slot(self.row_num)
    }
}