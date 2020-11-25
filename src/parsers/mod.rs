pub(crate) mod entry;
pub(crate) mod handshake;
pub(crate) mod parameters;

use nom::branch::alt;
use nom::bytes::streaming::{tag, take_while1};
use nom::combinator::map_res;
use nom::IResult;
use std::str::FromStr;

fn is_body_char(c: u8) -> bool {
    !(c as char).is_control()
}

fn is_parameter_char(c: u8) -> bool {
    is_body_char(c) && (c as char) != '|'
}

fn parse_string_parameter(input: &[u8]) -> IResult<&[u8], String> {
    let (input, s) = take_while1(is_parameter_char)(input)?;
    Ok((input, String::from_utf8(s.to_vec()).unwrap()))
}

fn parse_data_structure<T>(input: &[u8]) -> IResult<&[u8], T>
where
    T: FromStr,
{
    map_res(take_while1(is_parameter_char), |s: &[u8]| {
        T::from_str(&String::from_utf8_lossy(s))
    })(input)
}

fn parse_delimiter(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("|")(input)
}

fn parse_eol(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((tag("\r\n"), tag("\n")))(input)
}

fn parse_usize(input: &[u8]) -> IResult<&[u8], usize> {
    map_res(take_while1(|c| (c as char).is_ascii_digit()), |s| {
        usize::from_str_radix(&String::from_utf8_lossy(s), 10)
    })(input)
}

#[cfg(test)]
mod tests {
    use super::is_parameter_char;

    #[test]
    fn test_valid_parameter_char() {
        let char_lst = "a0.:-_/";
        for c in char_lst.bytes() {
            assert!(is_parameter_char(c));
        }
    }

    #[test]
    fn test_invalid_parameter_char() {
        let char_lst = "|\n";
        for c in char_lst.bytes() {
            assert!(!is_parameter_char(c));
        }
    }
}
