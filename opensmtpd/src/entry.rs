// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::errors::Error;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{digit1, hex_digit1, line_ending};
use nom::combinator::{map_res, value};
use nom::multi::many0;
use nom::IResult;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Version {
    V1,
}

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
    LinkAuth,
    LinkConnect,
    LinkDisconnect,
    LinkIdentify,
    LinkReset,
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
    FilterResponse,
    Timeout,
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let s = if !s.contains('-') {
            s.replace("link", "link-")
                .replace("tx", "tx-")
                .replace("protocol", "protocol-")
                .replace("filter", "filter-")
        } else {
            s
        };
        let (_, evt) = parse_event(&s)?;
        Ok(evt)
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        let s = match self {
            Event::LinkAuth => "link-auth",
            Event::LinkConnect => "link-connect",
            Event::LinkDisconnect => "link-disconnect",
            Event::LinkIdentify => "link-identify",
            Event::LinkReset => "link-reset",
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
            Event::FilterResponse => "filter-response",
            Event::Timeout => "timeout",
        };
        String::from(s)
    }
}

#[derive(Debug)]
pub struct TimeVal {
    pub sec: i64,
    pub usec: i64,
}

impl ToString for TimeVal {
    fn to_string(&self) -> String {
        format!("{}.{}", self.sec, self.usec)
    }
}

#[derive(Debug)]
pub enum Entry {
    V1Report(V1Report),
    V1Filter(V1Filter),
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(entry: &str) -> Result<Self, Self::Err> {
        let (_, res) = parse_entry(entry)?;
        Ok(res)
    }
}

impl Entry {
    pub fn get_event(&self) -> Event {
        match self {
            Entry::V1Report(r) => r.event.to_owned(),
            Entry::V1Filter(f) => f.event.to_owned(),
        }
    }

    pub fn get_session_id(&self) -> u64 {
        match self {
            Entry::V1Report(r) => r.session_id,
            Entry::V1Filter(f) => f.session_id,
        }
    }

    pub fn is_disconnect(&self) -> bool {
        match self {
            Entry::V1Report(r) => r.event == Event::LinkDisconnect,
            Entry::V1Filter(_) => false,
        }
    }
}

#[derive(Debug)]
pub struct V1Report {
    pub timestamp: TimeVal,
    pub subsystem: Subsystem,
    pub event: Event,
    pub session_id: u64,
    pub params: Vec<String>,
}

#[derive(Debug)]
pub struct V1Filter {
    pub timestamp: TimeVal,
    pub subsystem: Subsystem,
    pub event: Event,
    pub session_id: u64,
    pub token: u64,
    pub params: Vec<String>,
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

fn parse_version(input: &str) -> IResult<&str, Version> {
    value(Version::V1, tag("1"))(input)
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
        value(Event::LinkAuth, tag("link-auth")),
        value(Event::LinkConnect, tag("link-connect")),
        value(Event::LinkDisconnect, tag("link-disconnect")),
        value(Event::LinkIdentify, tag("link-identify")),
        value(Event::LinkReset, tag("link-reset")),
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
        value(Event::FilterResponse, tag("filter-response")),
        value(Event::Timeout, tag("timeout")),
    ))(input)
}

fn parse_token(input: &str) -> IResult<&str, u64> {
    map_res(hex_digit1, |s: &str| u64::from_str_radix(s, 16))(input)
}

fn parse_session_id(input: &str) -> IResult<&str, u64> {
    map_res(hex_digit1, |s: &str| u64::from_str_radix(s, 16))(input)
}

fn parse_param(input: &str) -> IResult<&str, String> {
    let (input, _) = separator(input)?;
    let (input, param) = take_till(is_end_param)(input)?;
    Ok((input, param.to_string()))
}

fn is_end_param(c: char) -> bool {
    c == '|' || c == '\r' || c == '\n'
}

fn parse_v1_report(input: &str) -> IResult<&str, Entry> {
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, _) = separator(input)?;
    let (input, subsystem) = parse_subsystem(input)?;
    let (input, _) = separator(input)?;
    let (input, event) = parse_event(input)?;
    let (input, _) = separator(input)?;
    let (input, session_id) = parse_session_id(input)?;
    let (input, params) = many0(parse_param)(input)?;
    let _ = line_ending(input)?;
    let report = V1Report {
        timestamp,
        subsystem,
        event,
        session_id,
        params,
    };
    Ok((input, Entry::V1Report(report)))
}

fn parse_v1_filter(input: &str) -> IResult<&str, Entry> {
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, _) = separator(input)?;
    let (input, subsystem) = parse_subsystem(input)?;
    let (input, _) = separator(input)?;
    let (input, event) = parse_event(input)?;
    let (input, _) = separator(input)?;
    let (input, token) = parse_token(input)?;
    let (input, _) = separator(input)?;
    let (input, session_id) = parse_session_id(input)?;
    let (input, params) = many0(parse_param)(input)?;
    let _ = line_ending(input)?;
    let filter = V1Filter {
        timestamp,
        subsystem,
        event,
        session_id,
        token,
        params,
    };
    Ok((input, Entry::V1Filter(filter)))
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, kind) = parse_kind(input)?;
    let (input, _) = separator(input)?;
    let (input, version) = parse_version(input)?;
    let (input, _) = separator(input)?;
    let (input, entry) = match version {
        Version::V1 => match kind {
            Kind::Report => parse_v1_report(input)?,
            Kind::Filter => parse_v1_filter(input)?,
        },
    };
    Ok((input, entry))
}
