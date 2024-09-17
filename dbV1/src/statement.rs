use crate::input_buffer::InputBuffer;
pub enum StatementType{ 
    StatementInsert, 
    StatementSelect,
    StatementUnrecognized
}

pub struct Statement {
    pub statement_type: StatementType
}

impl Statement {
    pub fn new() -> Statement {
        Statement{
            statement_type: StatementType::StatementUnrecognized
        }
    }
    pub fn prepare_statement(&mut self, input_buffer: &InputBuffer) -> StatementType{
        if input_buffer.buffer.contains("insert") {
            self.statement_type = StatementType::StatementInsert;
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
                println!("Something is wrong");
            }
        }
    }
}