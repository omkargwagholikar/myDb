use crate::constants::*;

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub struct Pager {
    pub file_descriptor: File,
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
            file_descriptor: file,
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
            let page = vec![0u8; PAGE_SIZE];
            let mut num_pages = self.file_length / PAGE_SIZE;
            if self.file_length % PAGE_SIZE != 0 {
                num_pages += 1;
            }
            println!("{} <= {}", page_num, num_pages);
            if page_num < num_pages {
                let mut page = vec![0u8; PAGE_SIZE];
                self.file_descriptor.seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64)).expect("Error in seeking to eof");
                self.file_descriptor.read_exact(&mut page).expect("Error in reading");
            }

            self.pages[page_num] = Some(page);
        }
        return  self.pages[page_num].as_mut().unwrap();
    }
    
    // pub fn get_page(&mut self, page_num: usize) -> &mut Vec<u8> {
    //     if page_num > TABLE_MAX_PAGES {
    //         println!("Tried to fetch page number out of bounds. {} > {}\n", page_num, TABLE_MAX_PAGES);
    //         std::process::exit(1);
    //     }
    //     if self.pages[page_num].is_none() {
    //         let mut page = vec![0u8; PAGE_SIZE];
    //         let mut num_pages = self.file_length / PAGE_SIZE;
    //         if self.file_length % PAGE_SIZE == 0 {
    //             num_pages += 1;
    //         }
    //         if page_num < num_pages {
    //             match self.file_descriptor.seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64)) {
    //                 Ok(_) => {
                        
    //                 },
    //                 Err(_) => {
    //                     println!("Error seeking in File");
    //                     std::process::exit(1);
    //                 },
    //             }
                
    //             match self.file_descriptor.read_exact(&mut page) {
    //                 Ok(_) => {                        
    //                     // Caching 
    //                     self.pages[page_num] = Some(page);
    //                 },
    //                 Err(e) => {
    //                     if e.kind() != std::io::ErrorKind::UnexpectedEof {
    //                         eprintln!("Problem in reading from the file: {}", e);
    //                         println!("Kind: {}", e.kind().to_string());
    //                         std::process::exit(1);
    //                     } 
    //                     println!("page_num: {} num_pages: {}", page_num, num_pages);
    //                     // if page_num == num_pages {
    //                         println!("Adding new page to the file");
    //                         self.pages[page_num] = Some(page);
    //                         // Extend the file by appending the new page
    //                         let page = vec![0u8; PAGE_SIZE];
    //                         match self.file_descriptor.write_all(&page) {
    //                             Ok(_) => {
    //                                 // Update file length
    //                                 self.file_length = self.file_descriptor.metadata().unwrap().len() as usize;
    //                             }
    //                             Err(err) => {
    //                                 eprintln!("Error writing new page to file: {}", err);
    //                                 std::process::exit(1);
    //                             }
    //                         }
    //                     // }
    //                 },
    //             }
    //         }
    //     }
    //     return  self.pages[page_num].as_mut().unwrap();
    // }

    pub fn flush(&mut self, page_num: usize, size: usize) {
        if self.pages[page_num].is_none() {
            println!("Tried to flush null page");
            return;
            // std::process::exit(1);
        }

        match self.file_descriptor.seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64)) {
            Ok(_) => {
                
            },
            Err(_) => {
                println!("Error seeking in File");
                std::process::exit(1);
            },
        }

        if let Some(page) = &self.pages[page_num] {
            match self.file_descriptor.write(&page[..size]) {
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
