use crate::constants::*;

pub struct Table{
    num_rows: i32,
    pages: [u8; TABLE_MAX_PAGES]
}
