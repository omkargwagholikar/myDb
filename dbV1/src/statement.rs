use crate::{input_buffer::InputBuffer, row::Row, constants::*};
pub enum StatementType{ 
    StatementInsert, 
    StatementSelect,
    StatementUnrecognized,
    PrepareSyntaxError
}

pub struct Statement {
    pub statement_type: StatementType,
    pub row: Row
}

impl Statement {
    pub fn new() -> Statement {
        Statement{
            statement_type: StatementType::StatementUnrecognized,
            row: Row::new()
        }
    }
    pub fn prepare_statement(&mut self, input_buffer: &InputBuffer) -> StatementType{
        if input_buffer.buffer.contains("insert") {
            self.statement_type = StatementType::StatementInsert;

            let a: Vec<String> = input_buffer.buffer.split_whitespace().map(str::to_string).collect();
    
            if let Some(bb) = a.get(1) {
                self.row.id = bb.parse::<i32>().unwrap_or(-1);
            } else {
                println!("Could not read ID properly");
                return StatementType::PrepareSyntaxError
            }
            
            if let Some(bb) = a.get(2) {
                for (i, c) in bb.chars().take(COLUMN_USERNAME_SIZE).enumerate() {
                    self.row.username[i] = c;
                }
            } else {
                println!("Could not read Username properly");
                return StatementType::PrepareSyntaxError
            }
            
            if let Some(bb) = a.get(3) {
                for (i, c) in bb.chars().take(COLUMN_EMAIL_SIZE).enumerate() {
                    self.row.email[i] = c;
                }
            } else {
                println!("Could not read Email properly");
                return StatementType::PrepareSyntaxError
            }

            return StatementType::StatementInsert
        }
        if input_buffer.buffer.contains("select") {
            self.statement_type = StatementType::StatementSelect;
            return  StatementType::StatementSelect;
        }
        StatementType::StatementUnrecognized
    }

    pub fn execute_statement(&mut self) {
        match self.statement_type {
            StatementType::StatementInsert => {
                println!("This is where we would do an insert.");
            },
            StatementType::StatementSelect => {
                println!("This is where we would do a select.");
            },
            StatementType::StatementUnrecognized => {
                println!("Statement Unrecognized");
            }
            StatementType::PrepareSyntaxError => {
                println!("Error in executing statement")
            },
        }
    }
}