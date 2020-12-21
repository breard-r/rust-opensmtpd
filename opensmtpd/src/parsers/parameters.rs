use super::{
    is_body_char, is_parameter_char, parse_data_structure, parse_delimiter, parse_eol,
    parse_string_parameter, parse_usize,
};
use crate::{Address, AuthResult, FilterKind, FilterPhase, MailResult, Method};
use nom::branch::alt;
use nom::bytes::streaming::{tag, take_while, take_while1};
use nom::combinator::{map_res, opt};
use nom::IResult;
use std::net::SocketAddr;
use std::path::PathBuf;

pub(crate) fn parse_filter_auth(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, s))
}

pub(crate) fn parse_filter_connect(
    input: &[u8],
) -> IResult<&[u8], (String, String, Address, Address)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, rdns) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, fcrdns) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, src) = parse_address(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, dest) = parse_address(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (rdns, fcrdns, src, dest)))
}

pub(crate) fn parse_filter_data_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = take_while(is_body_char)(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, s))
}

pub(crate) fn parse_filter_ehlo(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, s))
}

pub(crate) fn parse_filter_helo(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, s))
}

pub(crate) fn parse_filter_mail_from(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, s))
}

pub(crate) fn parse_filter_rcpt_to(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, s))
}

pub(crate) fn parse_filter_starttls(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, s))
}

pub(crate) fn parse_report_link_auth(input: &[u8]) -> IResult<&[u8], (String, AuthResult)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, username) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, result) = parse_data_structure::<AuthResult>(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (username, result)))
}

pub(crate) fn parse_report_link_connect(
    input: &[u8],
) -> IResult<&[u8], (String, String, Address, Address)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, rdns) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, fcrdns) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, src) = parse_address(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, dest) = parse_address(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (rdns, fcrdns, src, dest)))
}

pub(crate) fn parse_report_link_greeting(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, hostname) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, hostname))
}

pub(crate) fn parse_report_link_identify(input: &[u8]) -> IResult<&[u8], (Method, String)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, method) = parse_data_structure::<Method>(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, identity) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (method, identity)))
}

pub(crate) fn parse_report_link_tls(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, tls_string) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, tls_string))
}

pub(crate) fn parse_report_tx_begin(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, id) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, id))
}

pub(crate) fn parse_report_tx_mail(input: &[u8]) -> IResult<&[u8], (String, MailResult, String)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, id) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, result) = parse_data_structure::<MailResult>(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, addr) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (id, result, addr)))
}

pub(crate) fn parse_report_tx_reset(input: &[u8]) -> IResult<&[u8], Option<String>> {
    let (input, id) = opt(parse_tx_reset_opt)(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, id))
}

fn parse_tx_reset_opt(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, id) = parse_string_parameter(input)?;
    Ok((input, id))
}

pub(crate) fn parse_report_tx_rcpt(input: &[u8]) -> IResult<&[u8], (String, MailResult, String)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, id) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, result) = parse_data_structure::<MailResult>(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, addr) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (id, result, addr)))
}

pub(crate) fn parse_report_tx_envelope(input: &[u8]) -> IResult<&[u8], (String, String)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, msg) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, env) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (msg, env)))
}

pub(crate) fn parse_report_tx_data(input: &[u8]) -> IResult<&[u8], (String, MailResult)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, id) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, result) = parse_data_structure::<MailResult>(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (id, result)))
}

pub(crate) fn parse_report_tx_commit(input: &[u8]) -> IResult<&[u8], (String, usize)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, id) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, s) = parse_usize(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (id, s)))
}

pub(crate) fn parse_report_tx_rollback(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, id) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, id))
}

pub(crate) fn parse_report_protocol_client(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, cmd) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, cmd))
}

pub(crate) fn parse_report_protocol_server(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    let (input, res) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, res))
}

pub(crate) fn parse_report_filter_response(
    input: &[u8],
) -> IResult<&[u8], (FilterPhase, String, Option<String>)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, phase) = parse_data_structure::<FilterPhase>(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, res) = parse_string_parameter(input)?;
    let (input, param) = opt(parse_filter_response_opt)(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (phase, res, param)))
}

fn parse_filter_response_opt(input: &[u8]) -> IResult<&[u8], String> {
    let (input, _) = parse_delimiter(input)?;
    parse_string_parameter(input)
}

pub(crate) fn parse_report_filter_report(
    input: &[u8],
) -> IResult<&[u8], (FilterKind, String, String)> {
    let (input, _) = parse_delimiter(input)?;
    let (input, kind) = parse_data_structure::<FilterKind>(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, name) = parse_string_parameter(input)?;
    let (input, _) = parse_delimiter(input)?;
    let (input, message) = parse_string_parameter(input)?;
    let (input, _) = parse_eol(input)?;
    Ok((input, (kind, name, message)))
}

fn parse_address(input: &[u8]) -> IResult<&[u8], Address> {
    alt((parse_unix_socket, parse_socketaddr))(input)
}

fn parse_socketaddr(input: &[u8]) -> IResult<&[u8], Address> {
    map_res(
        take_while1(is_parameter_char),
        |s: &[u8]| -> Result<Address, String> {
            let s = String::from_utf8(s.to_vec()).map_err(|e| e.to_string())?;
            let addr = s.parse::<SocketAddr>().map_err(|e| e.to_string())?;
            Ok(Address::Ip(addr))
        },
    )(input)
}

fn parse_unix_socket(input: &[u8]) -> IResult<&[u8], Address> {
    let (input, _) = tag("unix:")(input)?;
    map_res(
        take_while1(is_parameter_char),
        |s: &[u8]| -> Result<Address, String> {
            let s = String::from_utf8(s.to_vec()).map_err(|e| e.to_string())?;
            let addr = s.parse::<PathBuf>().map_err(|e| e.to_string())?;
            Ok(Address::UnixSocket(addr))
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    use std::path::Path;

    #[test]
    fn test_ipv4_port() {
        let res = parse_address(b"199.185.178.25:33174|");
        assert!(res.is_ok());
        let (i, addr) = res.unwrap();
        assert_eq!(i, b"|");
        match addr {
            Address::Ip(addr) => {
                assert!(addr.is_ipv4());
                assert_eq!(addr.port(), 33174);
                assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::new(199, 185, 178, 25)));
            }
            Address::UnixSocket(_) => assert!(false),
        };
    }

    #[test]
    fn test_ipv6_port() {
        let res = parse_address(b"[2001:db8::42]:33174\n");
        assert!(res.is_ok());
        let (i, addr) = res.unwrap();
        assert_eq!(i, b"\n");
        match addr {
            Address::Ip(addr) => {
                assert!(addr.is_ipv6());
                assert_eq!(addr.port(), 33174);
                assert_eq!(
                    addr.ip(),
                    IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x42))
                );
            }
            Address::UnixSocket(_) => assert!(false),
        };
    }

    #[test]
    fn test_unix_socket() {
        let res = parse_address(b"unix:/var/something.sock|");
        assert!(res.is_ok());
        let (i, addr) = res.unwrap();
        assert_eq!(i, b"|");
        match addr {
            Address::UnixSocket(addr) => {
                assert_eq!(addr, Path::new("/var/something.sock").to_path_buf());
            }
            Address::Ip(_) => assert!(false),
        };
    }

    #[test]
    fn test_valid_parse_filter_auth() {
        let test_vectors = vec![
            ("|derp\n", "derp"),
            ("|derp.derpson@example.com\r\n", "derp.derpson@example.com"),
        ];
        for (test, ref_auth) in test_vectors {
            let res = parse_filter_auth(test.as_bytes());
            assert!(res.is_ok());
            let (input, auth) = res.unwrap();
            assert_eq!(input, b"");
            assert_eq!(auth, ref_auth.to_string());
        }
    }

    #[test]
    fn test_invalid_parse_filter_auth() {
        let test_vectors = vec!["|\n", "|\r\n", "|derp", "|derp|derpson\n"];
        for test in test_vectors {
            let res = parse_filter_auth(test.as_bytes());
            assert!(!res.is_ok());
        }
    }
}
