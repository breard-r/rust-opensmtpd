use super::{parse_data_structure, parse_delimiter, parse_string_parameter};
use crate::Event;
use crate::FilterPhase;
use crate::SubSystem;
use crate::TimeVal;
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::streaming::digit1;
use nom::combinator::map_res;
use nom::IResult;

pub struct ReportEntry {
	pub version: String,
	pub timestamp: TimeVal,
	pub subsystem: SubSystem,
	pub event: Event,
	pub session_id: String,
}

pub struct FilterEntry {
	pub version: String,
	pub timestamp: TimeVal,
	pub subsystem: SubSystem,
	pub phase: FilterPhase,
	pub session_id: String,
	pub token: String,
}

pub(crate) enum EntryOption {
	Report(ReportEntry),
	Filter(FilterEntry),
}

pub(crate) fn parse_entry(input: &[u8]) -> IResult<&[u8], EntryOption> {
	let (input, entry) = alt((parse_report_entry_meta, parse_filter_entry_meta))(input)?;
	Ok((input, entry))
}

fn parse_report_entry_meta(input: &[u8]) -> IResult<&[u8], EntryOption> {
	let (input, _) = tag("report")(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, entry) = parse_report_entry(input)?;
	Ok((input, EntryOption::Report(entry)))
}

fn parse_filter_entry_meta(input: &[u8]) -> IResult<&[u8], EntryOption> {
	let (input, _) = tag("filter")(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, entry) = parse_filter_entry(input)?;
	Ok((input, EntryOption::Filter(entry)))
}

fn parse_report_entry(input: &[u8]) -> IResult<&[u8], ReportEntry> {
	let (input, version) = parse_string_parameter(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, timestamp) = parse_timestamp(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, subsystem) = parse_data_structure::<SubSystem>(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, event) = parse_data_structure::<Event>(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, session_id) = parse_string_parameter(input)?;
	let entry = ReportEntry {
		version,
		timestamp,
		subsystem,
		event,
		session_id,
	};
	Ok((input, entry))
}

fn parse_filter_entry(input: &[u8]) -> IResult<&[u8], FilterEntry> {
	let (input, version) = parse_string_parameter(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, timestamp) = parse_timestamp(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, subsystem) = parse_data_structure::<SubSystem>(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, phase) = parse_data_structure::<FilterPhase>(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, session_id) = parse_string_parameter(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, token) = parse_string_parameter(input)?;
	let entry = FilterEntry {
		version,
		timestamp,
		subsystem,
		phase,
		session_id,
		token,
	};
	Ok((input, entry))
}

fn parse_timestamp(input: &[u8]) -> IResult<&[u8], TimeVal> {
	let (input, sec) = map_res(digit1, |s| String::from_utf8_lossy(s).parse::<i64>())(input)?;
	let (input, _) = tag(".")(input)?;
	let (input, usec) = map_res(digit1, |s| {
		format!("{:0<6}", String::from_utf8_lossy(s)).parse::<i64>()
	})(input)?;
	let timestamp = TimeVal { sec, usec };
	Ok((input, timestamp))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_report_link_connect() {
		let input = b"report|0.5|1576146008.06099|smtp-in|link-connect|7641df9771b4ed00|mail.openbsd.org|pass|199.185.178.25:33174|45.77.67.80:25";
		let res = parse_entry(input);
		assert!(res.is_ok());
		let (_, res) = res.unwrap();
		let res = match res {
			EntryOption::Report(r) => r,
			_ => {
				assert!(false);
				return;
			}
		};
		assert_eq!(res.version, String::from("0.5"));
		assert_eq!(
			res.timestamp,
			TimeVal {
				sec: 1576146008,
				usec: 60990
			}
		);
		assert_eq!(res.subsystem, SubSystem::SmtpIn);
		assert_eq!(res.event, Event::LinkConnect);
		assert_eq!(res.session_id, String::from("7641df9771b4ed00"));
	}
}
