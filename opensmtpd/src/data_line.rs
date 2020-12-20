use crate::FilterEntry;
use std::io::{self, Write};

pub fn return_data_line(entry: &FilterEntry, data_line: &[u8]) {
    print!("filter-dataline|{}|{}|", entry.session_id, entry.token);
    io::stdout().write_all(data_line).unwrap();
    println!("");
}
