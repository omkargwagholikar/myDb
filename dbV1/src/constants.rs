use std::mem::size_of;
#[allow(dead_code)]
pub const COLUMN_USERNAME_SIZE: usize = 32;
#[allow(dead_code)]
pub const COLUMN_EMAIL_SIZE: usize = 255;

#[allow(dead_code)]
pub const ID_SIZE: usize = size_of::<usize>() as usize;
#[allow(dead_code)]
pub const USERNAME_SIZE: usize = size_of::<[char; 32]>() as usize;
#[allow(dead_code)]
pub const EMAIL_SIZE: usize = size_of::<[char; 255]>() as usize;
#[allow(dead_code)]
pub const ID_OFFSET: usize = 0;
#[allow(dead_code)]
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
#[allow(dead_code)]
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
#[allow(dead_code)]
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;
#[allow(dead_code)]
pub const PAGE_SIZE: usize = 4096;
#[allow(dead_code)]
pub const TABLE_MAX_PAGES: usize = 100;
#[allow(dead_code)]
pub const ROWS_PER_PAGE: usize = (PAGE_SIZE / ROW_SIZE) as usize;
#[allow(dead_code)]
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;