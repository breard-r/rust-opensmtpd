use nom::error::Error;
use nom::Err;
use pretty_hex::pretty_hex;

fn error_to_string(e: Error<&[u8]>) -> String {
	format!(
		"parsing error: {:?}: input:{}",
		e.code,
		get_pretty_hex(&e.input)
	)
}

pub(crate) fn get_pretty_hex(input: &[u8]) -> String {
	let mut s = String::new();
	for l in pretty_hex(&input).split('\n') {
		s += &format!("\n{}", l);
	}
	s
}

pub(crate) fn nom_err_to_string(e: Err<Error<&[u8]>>) -> String {
	match e {
		Err::Incomplete(_) => e.to_string(),
		Err::Error(er) => error_to_string(er),
		Err::Failure(er) => error_to_string(er),
	}
}
