use crate::{constants::*, input_buffer::InputBuffer, row::Row, table::Table};
pub enum StatementType{ 
    StatementInsert, 
    StatementSelect,
    Default
}

pub enum PrepareResult {
    StatementUnrecognized,
    PrepareSyntaxError,
    PrepareSuccess

}

pub enum ExecuteResult {
    ExecuteTableFull,
    ExecuteSuccess
}

pub struct Statement {
    pub statement_type: StatementType,
    pub row: Row
}

impl Statement {
    pub fn new() -> Statement {
        Statement{
            statement_type: StatementType::Default,
            row: Row::new()
        }
    }
    pub fn prepare_statement(&mut self, input_buffer: &InputBuffer) -> PrepareResult{
        if input_buffer.buffer.contains("insert") {
            self.statement_type = StatementType::StatementInsert;

            let a: Vec<String> = input_buffer.buffer.split_whitespace().map(str::to_string).collect();
    
            if let Some(bb) = a.get(1) {
                self.row.id = bb.parse::<i32>().unwrap_or(-1);
            } else {
                println!("Could not read ID properly");
                return PrepareResult::PrepareSyntaxError
            }
            
            if let Some(bb) = a.get(2) {
                for (i, c) in bb.chars().take(COLUMN_USERNAME_SIZE).enumerate() {
                    self.row.username[i] = c;
                }
            } else {
                println!("Could not read Username properly");
                return PrepareResult::PrepareSyntaxError
            }
            
            if let Some(bb) = a.get(3) {
                for (i, c) in bb.chars().take(COLUMN_EMAIL_SIZE).enumerate() {
                    self.row.email[i] = c;
                }
            } else {
                println!("Could not read Email properly");
                return PrepareResult::PrepareSyntaxError
            }

            return PrepareResult::PrepareSuccess
        }
        if input_buffer.buffer.contains("select") {
            self.statement_type = StatementType::StatementSelect;
            return  PrepareResult::PrepareSuccess;
        }
        PrepareResult::StatementUnrecognized
    }

    pub fn execute_insert(&self, table: &mut Table) -> ExecuteResult {
        if table.num_rows as usize >= TABLE_MAX_ROWS {
            return ExecuteResult::ExecuteTableFull;
        }

        Row::serialize_row(&self.row, table.row_slot(table.num_rows));
        table.num_rows += 1;
        
        return ExecuteResult::ExecuteSuccess;
    }

    pub fn execute_select(&self, table: &mut Table) -> ExecuteResult {
        let mut row: Row = Row::new();
        println!("Id\tUsername\tEmail");
        for i in 0..table.num_rows {
            Row::deserialize_row(&table.row_slot(i), &mut row);
            row.print_row();
        }
        return  ExecuteResult::ExecuteSuccess;
    }

    pub fn execute_statement(&mut self, table: &mut Table) {
        match self.statement_type {
            StatementType::StatementInsert => {
                println!("Inserting...");
                self.execute_insert(table);
            },
            StatementType::StatementSelect => {
                println!("Selecting...");
                self.execute_select(table);
            },
            StatementType::Default => {
                println!("Statement not initalized yet");
            }
        }
    }
}