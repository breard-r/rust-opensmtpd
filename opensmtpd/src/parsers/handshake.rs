use super::{
	parse_data_structure, parse_delimiter, parse_eol, parse_string_parameter, parse_usize,
};
use crate::SubSystem;
use nom::bytes::streaming::tag;
use nom::IResult;

#[derive(Debug)]
pub(crate) struct Handshake {
	pub(crate) smtpd_version: String,
	pub(crate) smtp_session_timeout: usize,
	pub(crate) subsystem: SubSystem,
}

pub(crate) fn parse_handshake(input: &[u8]) -> IResult<&[u8], Handshake> {
	let (input, smtpd_version) = parse_smtpd_version(input)?;
	let (input, smtp_session_timeout) = parse_smtp_session_timeout(input)?;
	let (input, subsystem) = parse_subsystem(input)?;
	let (input, _) = parse_ready(input)?;
	let handshake = Handshake {
		smtpd_version,
		smtp_session_timeout,
		subsystem,
	};
	Ok((input, handshake))
}

fn parse_smtpd_version(input: &[u8]) -> IResult<&[u8], String> {
	let (input, _) = parse_config_initial(input)?;
	let (input, _) = tag("smtpd-version")(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, version) = parse_string_parameter(input)?;
	let (input, _) = parse_eol(input)?;
	Ok((input, version))
}

fn parse_smtp_session_timeout(input: &[u8]) -> IResult<&[u8], usize> {
	let (input, _) = parse_config_initial(input)?;
	let (input, _) = tag("smtp-session-timeout")(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, timeout) = parse_usize(input)?;
	let (input, _) = parse_eol(input)?;
	Ok((input, timeout))
}

fn parse_subsystem(input: &[u8]) -> IResult<&[u8], SubSystem> {
	let (input, _) = parse_config_initial(input)?;
	let (input, _) = tag("subsystem")(input)?;
	let (input, _) = parse_delimiter(input)?;
	let (input, subsystem) = parse_data_structure::<SubSystem>(input)?;
	let (input, _) = parse_eol(input)?;
	Ok((input, subsystem))
}

fn parse_ready(input: &[u8]) -> IResult<&[u8], ()> {
	let (input, _) = parse_config_initial(input)?;
	let (input, _) = tag("ready")(input)?;
	let (input, _) = parse_eol(input)?;
	Ok((input, ()))
}

fn parse_config_initial(input: &[u8]) -> IResult<&[u8], ()> {
	let (input, _) = tag("config")(input)?;
	let (input, _) = parse_delimiter(input)?;
	Ok((input, ()))
}

#[cfg(test)]
mod tests {
	use super::parse_handshake;
	use crate::SubSystem;

	#[test]
	fn test_valid_handshake_nl() {
		let input = b"config|smtpd-version|6.6.1\nconfig|smtp-session-timeout|300\nconfig|subsystem|smtp-in\nconfig|ready\n";
		let r = parse_handshake(input);
		assert!(r.is_ok());
		let (r, h) = r.unwrap();
		assert_eq!(r, b"");
		assert_eq!(h.smtpd_version, "6.6.1");
		assert_eq!(h.smtp_session_timeout, 300);
		assert_eq!(h.subsystem, SubSystem::SmtpIn);
	}

	#[test]
	fn test_valid_handshake_crnl() {
		let input = b"config|smtpd-version|6.6.1\r\nconfig|smtp-session-timeout|300\r\nconfig|subsystem|smtp-in\r\nconfig|ready\r\n";
		let r = parse_handshake(input);
		assert!(r.is_ok());
		let (r, h) = r.unwrap();
		assert_eq!(r, b"");
		assert_eq!(h.smtpd_version, "6.6.1");
		assert_eq!(h.smtp_session_timeout, 300);
		assert_eq!(h.subsystem, SubSystem::SmtpIn);
	}

	#[test]
	fn test_valid_handshake_over() {
		let input = b"config|smtpd-version|6.6.1\nconfig|smtp-session-timeout|300\nconfig|subsystem|smtp-in\nconfig|ready\nplop";
		let r = parse_handshake(input);
		assert!(r.is_ok());
		let (r, h) = r.unwrap();
		assert_eq!(r, b"plop");
		assert_eq!(h.smtpd_version, "6.6.1");
		assert_eq!(h.smtp_session_timeout, 300);
		assert_eq!(h.subsystem, SubSystem::SmtpIn);
	}

	#[test]
	fn test_invalid_handshakes() {
		let test_vectors = vec![
            "config|smtpd-version|6.6.1\nconfig|smtp-session-timeout|\nconfig|subsystem|smtp-in\nconfig|ready\n",
            "config|smtp-session-timeout|300\nconfig|smtpd-version|6.6.1\nconfig|subsystem|smtp-in\nconfig|ready\n",
            "config|smtpd-version|6.6.1\nconfig|smtp-session-timeout|300\nconfig|subsystem|smtp-in\nconfig|ready",
            "config|smtpd-version|6.6.1\nconfig|smtp-session-timeout|300\nconfig|subsystem|smtp-in\nconfig|ready\r",
        ];
		for input in test_vectors {
			let r = parse_handshake(input.as_bytes());
			assert!(r.is_err());
		}
	}
}
