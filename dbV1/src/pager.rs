use crate::constants::*;

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub struct Pager {
    pub file: File,
    pub file_length: usize,
    pub pages: [Option<Vec<u8>>; TABLE_MAX_PAGES],
}

impl Pager {
    pub fn new(file_name: &str) -> Pager {
        let file = 
            match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(file_name)
            {
                Ok(f) => f,
                Err(_) => {
                    println!("Unable to open file");
                    std::process::exit(1);
                }
            };

        let file_length = 
            match file.metadata() {
                Ok(metadata) => metadata.len() as usize, 
                Err(_) => {
                    println!("Unable to get file length");
                    std::process::exit(1);
                }
            };
        let pages = std::array::from_fn(|_| None);

        Pager {
            file,
            file_length,
            pages,
        }
    }

    pub fn get_page(&mut self, page_num: usize) -> &mut Vec<u8> {
        if page_num > TABLE_MAX_PAGES {
            println!("Tried to fetch page number out of bounds. {} > {}\n", page_num, TABLE_MAX_PAGES);
            std::process::exit(1);            
        }

        if self.pages[page_num].is_none() {
            let mut page = vec![0u8; PAGE_SIZE];
            let num_pages = self.file_length / PAGE_SIZE;

            if page_num < num_pages {
                self.file.seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64)).expect("Error in seeking to eof");
                self.file.read_exact(&mut page).expect("Error in reading");
            } else {
                self.file.seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64)).expect("Error in seeking to eof");
                self.file.read(&mut page).expect("Error in reading partially complete page");
            }

            self.pages[page_num] = Some(page);
        }
        return  self.pages[page_num].as_mut().unwrap();
    }

    pub fn flush(&mut self, page_num: usize, size: usize) {
        if self.pages[page_num].is_none() {
            println!("Tried to flush null page");
            return;
        }

        match self.file.seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64)) {
            Ok(_) => {
                
            },
            Err(_) => {
                println!("Error seeking in File");
                std::process::exit(1);
            },
        }

        if let Some(page) = &self.pages[page_num] {
            match self.file.write(&page[..size]) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Error writing: {}", err);
                    std::process::exit(1);
                }
            }
        } else {
            println!("Flush issue");
        }
    }
}
