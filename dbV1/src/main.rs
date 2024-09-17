mod input_buffer;
mod statement;
mod row;
mod constants;
mod table;

use input_buffer::InputBuffer;
use statement::Statement;

use std::process::exit;
use std::io::{stdout, Write};

enum MetaCommandResult{
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

fn print_prompt() {
    print!("dbv1 > ");
    stdout().flush().unwrap();
}

fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult{
    if input_buffer.buffer == ".exit" {
        exit(0);
    } else if input_buffer.buffer == ".delete" {
        MetaCommandResult::MetaCommandSuccess
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn main() {    
    loop {
        print_prompt();
        let mut input_buffer = InputBuffer::new();
        input_buffer.read_input();
        let mut statement = Statement::new();
        statement.prepare_statement(&input_buffer);                

        if input_buffer.buffer.chars().nth(0).unwrap() == '.' {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command: '{}'", input_buffer.buffer);
                    continue;
                }
            }
        } else {
            let mut statement = Statement::new();
            statement.prepare_statement(&input_buffer);
            statement.execute_statement();
            println!("Execution complete of: {}", input_buffer.buffer);
        }
    }
}

// let row = statement.row;
// let mut buffer: Vec<u8> = vec![0; ID_SIZE + USERNAME_SIZE + EMAIL_SIZE];
// Row::serialize_row(&row, &mut buffer);
// let mut deSerialRow = Row::new();
// Row::deserialize_row(&buffer, &mut deSerialRow);