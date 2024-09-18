mod input_buffer;
mod statement;
mod row;
mod constants;
mod table;
mod pager;

use input_buffer::InputBuffer;
use statement::Statement;
use table::Table;

use constants::*;

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
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn main() {
    
    let file_name = String::from("temp2.db");
    let mut table = Table::new(&file_name);
    println!("Table has: {} rows", table.num_rows);

    // for _i in 0..3 {
    //     let mut input_buffer = InputBuffer::new();
    //     input_buffer.buffer = "insert 1 omkar wagholikar".to_owned();
    //     let mut statement =  Statement::new();
    //     statement.prepare_statement(&input_buffer);
    //     statement.execute_statement(&mut table);
    // }

    // let a: f32 = table.pager.file_length as f32;
    // let b: f32 = ROW_SIZE as f32;
    // println!("This is number of rows: {}", ((a)/b));

    // table.db_close();

    // Row::serialize_row(&r, &mut table.row_slot(0));
    // Row::deserialize_row(&table.row_slot(0), &mut s);
    // s.print_row();
    // Row::serialize_row(&r, &mut ser);
    // Row::deserialize_row(&ser, &mut s);

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
