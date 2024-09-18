use crate::constants::*;
use std::ptr;

pub struct Row {
    // id, username, email
    pub id: i32,
    pub username: [char; COLUMN_USERNAME_SIZE],
    pub email: [char; COLUMN_EMAIL_SIZE]
}

impl Row {
    pub fn new() -> Row{
        Row {
            id: -1, 
            username: ['\0'; COLUMN_USERNAME_SIZE],
            email: ['\0'; COLUMN_EMAIL_SIZE]
        }
    }

    pub fn serialize_row(source: &Row, destination: &mut [u8]) {
        unsafe {
            // Copy the id to the destination
            ptr::copy_nonoverlapping(
                &source.id as *const i32 as *const u8,
                destination.as_mut_ptr().add(ID_OFFSET),
                ID_SIZE,
            );
            
            // Copy the username to the destination
            ptr::copy_nonoverlapping(
                source.username.as_ptr() as *const u8,
                destination.as_mut_ptr().add(USERNAME_OFFSET),
                USERNAME_SIZE,
            );
            
            // Copy the email to the destination
            ptr::copy_nonoverlapping(
                source.email.as_ptr() as *const u8,
                destination.as_mut_ptr().add(EMAIL_OFFSET),
                EMAIL_SIZE,
            );
        }
    }
    
    pub fn deserialize_row(source: &[u8], destination: &mut Row) {
        unsafe {
            // Copy the id from the source
            ptr::copy_nonoverlapping(
                source.as_ptr().add(ID_OFFSET),
                &mut destination.id as *mut i32 as *mut u8,
                ID_SIZE,
            );
    
            // Copy the username from the source
            ptr::copy_nonoverlapping(
                source.as_ptr().add(USERNAME_OFFSET),
                destination.username.as_mut_ptr() as *mut u8,
                USERNAME_SIZE,
            );
    
            // Copy the email from the source
            ptr::copy_nonoverlapping(
                source.as_ptr().add(EMAIL_OFFSET),
                destination.email.as_mut_ptr() as *mut u8,
                EMAIL_SIZE,
            );
        }
    }
    pub fn print_row(&self) {
        let username: String = self.username.iter().collect();
        let email: String = self.email.iter().collect();
        println!("{}\t{}\t\t{}", self.id, username, email);
    }
}