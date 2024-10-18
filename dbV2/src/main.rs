mod input_buffer;
mod statement;
mod row;
mod constants;
mod table;
mod pager;
mod cursor;
mod node;
mod leaf_node;
mod internal_node;

use input_buffer::InputBuffer;
use node::Node;
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
        table.db_close();
        exit(0);
    } else if input_buffer.buffer == ".drop" {
        table.db_close();
        println!("Table dropped");
        MetaCommandResult::MetaCommandSuccess
    } else if input_buffer.buffer == ".btree" {
        println!("Tree: ");
        Node::print_tree(&mut table.pager, 0, 0);
        println!("Tree complete");
        MetaCommandResult::MetaCommandSuccess
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn main() {
    
    let file_name = String::from("test.db");
    let mut table = Table::new(&file_name);
    
    loop {
        print_prompt();
        let mut input_buffer = InputBuffer::new();
        input_buffer.read_input();
        let mut statement = Statement::new();
        statement.prepare_statement(&input_buffer);                
        if input_buffer.buffer.len() == 0 {
            continue;
        }
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
