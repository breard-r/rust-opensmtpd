use crate::{return_data_line, FilterEntry};

const EOF_CHAR: u8 = 0x2E;

macro_rules! handle_eom {
	($self: ident, $data_line: ident) => {
		if $data_line.len() == 1 && $data_line[0] == EOF_CHAR {
			$self.state = State::Done;
			return Status::Ok;
		}
	}
}

pub enum Error {
	AlreadyComplete,
}

pub enum Status {
	Ok,
	Incomplete,
	Error(Error),
}

enum State {
	Headers,
	Done,
}

pub struct MessageAggregator {
	pub headers: Vec<(String, Vec<u8>, Vec<u8>)>,
	state: State,
	entry: FilterEntry,
}

impl MessageAggregator {
	pub fn new(entry: &FilterEntry) -> Self {
		MessageAggregator {
			headers: Vec::new(),
			state: State::Headers,
			entry: (*entry).clone(),
		}
	}

	pub fn return_message(&self) {
		let mut line = Vec::with_capacity(crate::BUFFER_SIZE);
		for (h_name, h_sep, h_value) in &self.headers {
			line.clear();
			line.extend_from_slice(h_name.as_bytes());
			line.extend_from_slice(&h_sep);
			line.extend_from_slice(&h_value);
			return_data_line(&self.entry, &line);
		}
		// TODO: return non-headers parts
		return_data_line(&self.entry, &[EOF_CHAR]);
	}

	pub fn append(&mut self, data_line: &[u8]) -> Status {
		match self.state {
			State::Headers => self.append_header(data_line),
			State::Done => Status::Error(Error::AlreadyComplete),
		}
	}

	fn append_header(&mut self, data_line: &[u8]) -> Status {
		handle_eom!(self, data_line);
		// TODO: parse the data_line and put it in self.headers
		Status::Incomplete
	}
}
