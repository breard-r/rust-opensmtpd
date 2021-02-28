use crate::FilterEntry;
use std::io::{self, Write};

pub fn return_data_line(entry: &FilterEntry, data_line: &[u8]) {
	let mut data_line = data_line.to_vec();
	data_line.retain(|&c| c != 0x0d && c != 0x0a);
	print!("filter-dataline|{}|{}|", entry.session_id, entry.token);
	io::stdout().write_all(&data_line).unwrap();
	println!();
	log::trace!(
		"Sent filter-dataline (session:id: {}, token: {}){}",
		entry.session_id,
		entry.token,
		crate::error::get_pretty_hex(&data_line)
	);
}
