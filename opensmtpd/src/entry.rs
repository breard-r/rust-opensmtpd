use crate::errors::Error;
use nom::{alt, alt_complete, call, complete, cond, do_parse, error_position, map_res, named, opt,
          tag, take_until, take_while};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Report,
    Filter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Subsystem {
    SmtpIn,
    SmtpOut,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    LinkConnect,
    LinkDisconnect,
    LinkIdentify,
    LinkTls,
    TxBegin,
    TxMail,
    TxRcpt,
    TxEnvelope,
    TxData,
    TxCommit,
    TxRollback,
    ProtocolClient,
    ProtocolServer,
    Timeout,
    FilterResponse,
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase()
            .replace("link", "link-")
            .replace("tx", "tx-")
            .replace("protocol", "protocol-")
            .replace("filter", "filter-");
        let (_, evt) = parse_event(&s)?;
        Ok(evt)
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        let s = match self {
            Event::LinkConnect => "link-connect",
            Event::LinkDisconnect => "link-disconnect",
            Event::LinkIdentify => "link-identify",
            Event::LinkTls => "link-tls",
            Event::TxBegin => "tx-begin",
            Event::TxMail => "tx-mail",
            Event::TxRcpt => "tx-rcpt",
            Event::TxEnvelope => "tx-envelope",
            Event::TxData => "tx-data",
            Event::TxCommit => "tx-commit",
            Event::TxRollback => "tx-rollback",
            Event::ProtocolClient => "protocol-client",
            Event::ProtocolServer => "protocol-server",
            Event::Timeout => "timeout",
            Event::FilterResponse => "filter-response",
        };
        String::from(s)
    }
}

#[derive(Debug)]
pub struct TimeVal {
    pub sec: i64,
    pub usec: i64,
}

impl TimeVal {
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.sec, self.usec)
    }
}

#[derive(Debug)]
pub struct Entry {
    pub kind: Kind,
    pub version: u8,
    pub timestamp: TimeVal,
    pub subsystem: Subsystem,
    pub event: Event,
    pub token: Option<u64>,
    pub session_id: u64,
    pub params: Option<String>,
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let (_, res) = parse_entry(entry)?;
        Ok(res)
    }
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_ascii_digit_or_neg(c: char) -> bool {
    c.is_ascii_digit() || c == '-'
}

fn is_ascii_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn to_u8(s: &str) -> Result<u8, std::num::ParseIntError> {
    s.parse()
}

fn to_i64(s: &str) -> Result<i64, std::num::ParseIntError> {
    s.parse()
}

fn to_u64_hex(s: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(s, 16)
}

named!(parse_i64<&str, i64>,
    map_res!(take_while!(is_ascii_digit_or_neg), to_i64)
);

named!(parse_u64_hex<&str, u64>,
    map_res!(take_while!(is_ascii_hexdigit), to_u64_hex)
);

named!(parse_kind<&str, Kind>,
    alt_complete!(
        tag!("report") => { |_| Kind::Report } |
        tag!("filter") => { |_| Kind::Filter }
    )
);

named!(parse_version<&str, u8>,
    map_res!(take_while!(is_ascii_digit), to_u8)
);

named!(parse_timestamp<&str, TimeVal>,
    do_parse!(
        sec: parse_i64 >>
        tag!(".") >>
        usec: parse_i64 >>
        (TimeVal { sec, usec})
    )
);

named!(parse_subsystem<&str, Subsystem>,
    alt_complete! (
        tag!("smtp-in") => { |_| Subsystem::SmtpIn } |
        tag!("smtp-out") => { |_| Subsystem::SmtpOut }
    )
);

named!(parse_event<&str, Event>,
    alt_complete!(
        tag!("link-connect") => { |_| Event::LinkConnect } |
        tag!("link-disconnect") => { |_| Event::LinkDisconnect } |
        tag!("link-identify") => { |_| Event::LinkIdentify } |
        tag!("link-tls") => { |_| Event::LinkTls } |
        tag!("tx-begin") => { |_| Event::TxBegin } |
        tag!("tx-mail") => { |_| Event::TxMail } |
        tag!("tx-rcpt") => { |_| Event::TxRcpt } |
        tag!("tx-envelope") => { |_| Event::TxEnvelope } |
        tag!("tx-data") => { |_| Event::TxData } |
        tag!("tx-commit") => { |_| Event::TxCommit } |
        tag!("tx-rollback") => { |_| Event::TxRollback } |
        tag!("protocol-client") => { |_| Event::ProtocolClient } |
        tag!("protocol-server") => { |_| Event::ProtocolServer } |
        tag!("timeout") => { |_| Event::Timeout } |
        tag!("filter-response") => { |_| Event::FilterResponse }
    )
);

named!(parse_token<&str, u64>,
    do_parse!(
        token: parse_u64_hex >>
        tag!("|") >>
        (token)
    )
);

named!(parse_params<&str, String>,
    do_parse!(
        tag!("|") >>
        s: take_until!("\n") >>
        (s.to_string())
    )
);

named!(parse_entry<&str, Entry>,
    do_parse!(
        kind: parse_kind >>
        tag!("|") >>
        version: parse_version >>
        tag!("|") >>
        timestamp: parse_timestamp >>
        tag!("|") >>
        subsystem: parse_subsystem >>
        tag!("|") >>
        event: parse_event >>
        tag!("|") >>
        token: cond!(kind == Kind::Filter, parse_token) >>
        session_id: parse_u64_hex >>
        params: opt!(parse_params) >>
        (Entry {
            kind,
            version,
            timestamp,
            subsystem,
            event,
            token,
            session_id,
            params,
        })
    )
);
