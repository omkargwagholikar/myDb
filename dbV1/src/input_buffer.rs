use std::io::stdin;

pub struct InputBuffer {
    pub buffer: String,
    pub buffer_length: usize,
    pub input_length: isize,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
            buffer_length: 0,
            input_length: -1,
        }
    }
    
    pub fn read_input(&mut self) {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(bytes_read) => {
                self.buffer = input.trim().to_string();
                self.buffer_length = self.buffer.len();
                self.input_length = bytes_read as isize;
            }
            Err(_) => {
                println!("Error reading input");
            }
        }
    }
}