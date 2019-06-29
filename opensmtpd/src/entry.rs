// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::errors::Error;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, hex_digit1, line_ending};
use nom::combinator::{map_res, opt, value};
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;
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
        let s = s
            .to_lowercase()
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

fn separator(input: &str) -> IResult<&str, &str> {
    tag("|")(input)
}

fn parse_kind(input: &str) -> IResult<&str, Kind> {
    alt((
        value(Kind::Report, tag("report")),
        value(Kind::Filter, tag("filter")),
    ))(input)
}

fn parse_version(input: &str) -> IResult<&str, u8> {
    map_res(digit1, |s: &str| s.parse::<u8>())(input)
}

fn parse_timestamp(input: &str) -> IResult<&str, TimeVal> {
    let (input, sec) = map_res(digit1, |s: &str| s.parse::<i64>())(input)?;
    let (input, _) = tag(".")(input)?;
    let (input, usec) = map_res(digit1, |s: &str| s.parse::<i64>())(input)?;
    let timestamp = TimeVal { sec, usec };
    Ok((input, timestamp))
}

fn parse_subsystem(input: &str) -> IResult<&str, Subsystem> {
    alt((
        value(Subsystem::SmtpIn, tag("smtp-in")),
        value(Subsystem::SmtpOut, tag("smtp-out")),
    ))(input)
}

fn parse_event(input: &str) -> IResult<&str, Event> {
    alt((
        value(Event::LinkConnect, tag("link-connect")),
        value(Event::LinkDisconnect, tag("link-disconnect")),
        value(Event::LinkIdentify, tag("link-identify")),
        value(Event::LinkTls, tag("link-tls")),
        value(Event::TxBegin, tag("tx-begin")),
        value(Event::TxMail, tag("tx-mail")),
        value(Event::TxRcpt, tag("tx-rcpt")),
        value(Event::TxEnvelope, tag("tx-envelope")),
        value(Event::TxData, tag("tx-data")),
        value(Event::TxCommit, tag("tx-commit")),
        value(Event::TxRollback, tag("tx-rollback")),
        value(Event::ProtocolClient, tag("protocol-client")),
        value(Event::ProtocolServer, tag("protocol-server")),
        value(Event::Timeout, tag("timeout")),
        value(Event::FilterResponse, tag("filter-response")),
    ))(input)
}

fn parse_token(input: &str) -> IResult<&str, u64> {
    map_res(hex_digit1, |s: &str| u64::from_str_radix(s, 16))(input)
}

fn parse_session_id(input: &str) -> IResult<&str, u64> {
    map_res(hex_digit1, |s: &str| u64::from_str_radix(s, 16))(input)
}

fn parse_params(input: &str) -> IResult<&str, String> {
    let (input, params) = many_till(anychar, line_ending)(input)?;
    Ok((input, params.0.into_iter().collect()))
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, kind) = parse_kind(input)?;
    let (input, _) = separator(input)?;
    let (input, version) = parse_version(input)?;
    let (input, _) = separator(input)?;
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, _) = separator(input)?;
    let (input, subsystem) = parse_subsystem(input)?;
    let (input, _) = separator(input)?;
    let (input, event) = parse_event(input)?;
    let (input, token) = if kind == Kind::Filter {
        let (input, _) = separator(input)?;
        let (input, token) = parse_token(input)?;
        (input, Some(token))
    } else {
        (input, None)
    };
    let (input, _) = separator(input)?;
    let (input, session_id) = parse_session_id(input)?;
    let (input, params) = opt(preceded(separator, parse_params))(input)?;
    if params.is_none() {
        let _ = line_ending(input)?;
    }
    let entry = Entry {
        kind,
        version,
        timestamp,
        subsystem,
        event,
        token,
        session_id,
        params,
    };
    Ok((input, entry))
}
