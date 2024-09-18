mod input_buffer;
mod statement;
mod row;
mod constants;
mod table;

use input_buffer::InputBuffer;
use statement::Statement;
use table::Table;

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

fn do_meta_command(input_buffer: &InputBuffer, table: &mut Table) -> MetaCommandResult{
    if input_buffer.buffer == ".exit" {
        exit(0);
    } else if input_buffer.buffer == ".drop" {
        table.drop_table();
        println!("Table dropped");
        MetaCommandResult::MetaCommandSuccess
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn main() { 
    let mut table = Table::new();
    
    loop {
        print_prompt();
        let mut input_buffer = InputBuffer::new();
        input_buffer.read_input();
        let mut statement = Statement::new();
        statement.prepare_statement(&input_buffer);                

        if input_buffer.buffer.chars().nth(0).unwrap() == '.' {
            match do_meta_command(&input_buffer, &mut table) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command: '{}'", input_buffer.buffer);
                    continue;
                }
            }
        } else {
            let mut statement = Statement::new();
            match statement.prepare_statement(&input_buffer) {
                statement::PrepareResult::StatementUnrecognized => {
                    println!("Statement not recognized");
                    continue
                },
                statement::PrepareResult::PrepareSyntaxError => {
                    println!("Syntax error. Could not parse statement.");
                    continue;
                },
                statement::PrepareResult::PrepareSuccess => {
                    
                },
                statement::PrepareResult::PrepareStringTooLong => {
                    println!("String is too long.");
                    continue;
                }
                statement::PrepareResult::PrepareNegativeId => {
                    println!("ID must be positive.");
                    continue;
                }
            }
            statement.execute_statement(&mut table);
            println!("Executed. ");
        }
    }
}