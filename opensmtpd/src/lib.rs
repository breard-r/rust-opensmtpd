//! [![Build Status](https://travis-ci.org/breard-r/rust-opensmtpd.svg?branch=main)](https://travis-ci.org/breard-r/rust-opensmtpd)
//! [![Rust-OpenSMTPD on crates.io](https://img.shields.io/crates/v/opensmtpd.svg)](https://crates.io/crates/opensmtpd)
//! [![Rust-OpenSMTPD on docs.rs](https://docs.rs/opensmtpd/badge.svg)](https://docs.rs/opensmtpd/)
//! ![License: MIT or Apache-2.0](https://img.shields.io/crates/l/opensmtpd)
//!
//! # Writing a filter for OpenSMTPD
//!
//! The first step is to define an object (most of the time you want
//! a struct) that implements the [`Filter`] trait. All of this
//! trait's methods have an empty default implementation, so you only
//! have to implement the ones that matters to you. For each method
//! you implement, you must use the [`opensmtpd_derive::register`]
//! attribute macro in order to ask OpenSMTPD to send you the
//! corresponding events and filter requests.
//!
//! The second and last step is to call the [`run_filter`] function
//! with a mutable reference of your filter object.
//!
//! # Examples
//!
//! The following filter increments a variable every time a client
//! disconnects.
//!
//! ```
//! use opensmtpd::{run_filter, Filter, ReportEntry};
//! use opensmtpd_derive::register;
//!
//! struct MyCounter {
//!     nb: u64,
//! }
//!
//! impl Filter for MyCounter {
//!     #[register]
//!     fn on_report_link_disconnect(&mut self, _entry: &ReportEntry) {
//!         self.nb + 1;
//!     }
//! }
//!
//! fn main() {
//!     let mut my_counter = MyCounter { nb: 0, };
//!     run_filter(&mut my_counter);
//! }
//! ```
//!
//! More examples can be found in the [examples directory](https://github.com/breard-r/rust-opensmtpd/tree/main/opensmtpd/examples).

mod data_line;
mod data_structures;
mod error;
mod filter;
mod io;
mod parsers;
mod process;

pub use crate::data_line::return_data_line;
pub use crate::data_structures::address::Address;
pub use crate::data_structures::auth_result::AuthResult;
pub use crate::data_structures::event::Event;
pub use crate::data_structures::filter_kind::FilterKind;
pub use crate::data_structures::filter_phase::FilterPhase;
pub use crate::data_structures::filter_response::FilterResponse;
pub use crate::data_structures::mail_result::MailResult;
pub use crate::data_structures::method::Method;
pub use crate::data_structures::smtp_status::SmtpStatusCode;
pub use crate::data_structures::subsystem::SubSystem;
pub use crate::data_structures::timeval::TimeVal;
pub use crate::filter::Filter;
pub use crate::parsers::entry::{FilterEntry, ReportEntry};

use crate::parsers::handshake::parse_handshake;
use std::sync::mpsc::channel;
use std::thread;

const BUFFER_SIZE: usize = 4096;

macro_rules! recv {
    ($rx: ident) => {
        match $rx.recv() {
            Ok(b) => b,
            Err(e) => {
                log::error!("{}", e);
                return;
            }
        }
    };
}

pub fn run_filter<T>(user_object: &mut T)
where
    T: Filter,
{
    // IO init
    let (tx, rx) = channel::<Vec<u8>>();
    thread::spawn(move || {
        io::read_stdin(&tx);
    });

    // Handshake
    let mut handshake_buffer: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
    let handshake = loop {
        let buffer = recv!(rx);
        handshake_buffer.extend_from_slice(&buffer);
        if let Ok((_, handshake)) = parse_handshake(&handshake_buffer) {
            break handshake;
        }
    };
    handshake_reply(user_object, handshake.subsystem);

    // Read and process input
    loop {
        let buffer = recv!(rx);
        if let Err(msg) = process::line(user_object, &buffer) {
            log::error!("{}", msg);
        }
    }
}

macro_rules! handshake_register {
    ($obj: ident, $func: ident, $subsystem: expr, $type: expr, $name: expr) => {
        if $obj.$func() {
            println!("register|{}|{}|{}", $type, $subsystem.to_string(), $name);
            log::trace!(
                "{} {} for {} registered",
                $type,
                $name,
                $subsystem.to_string()
            );
        }
    };
}

fn handshake_reply<T>(obj: &mut T, ss: SubSystem)
where
    T: Filter,
{
    // Filters
    handshake_register!(obj, has_filter_auth, ss, "report", "auth");
    handshake_register!(obj, has_filter_commit, ss, "report", "commit");
    handshake_register!(obj, has_filter_connect, ss, "report", "connect");
    handshake_register!(obj, has_filter_data, ss, "report", "data");
    handshake_register!(obj, has_filter_data_line, ss, "report", "data-line");
    handshake_register!(obj, has_filter_ehlo, ss, "report", "ehlo");
    handshake_register!(obj, has_filter_helo, ss, "report", "helo");
    handshake_register!(obj, has_filter_mail_from, ss, "report", "mail-from");
    handshake_register!(obj, has_filter_rcpt_to, ss, "report", "rcpt-to");
    handshake_register!(obj, has_filter_starttls, ss, "report", "starttls");

    // Reports
    handshake_register!(obj, has_report_link_auth, ss, "report", "link-auth");
    handshake_register!(obj, has_report_link_connect, ss, "report", "link-connect");
    handshake_register!(
        obj,
        has_report_link_disconnect,
        ss,
        "report",
        "link-disconnect"
    );
    handshake_register!(obj, has_report_link_greeting, ss, "report", "link-greeting");
    handshake_register!(obj, has_report_link_identify, ss, "report", "link-identify");
    handshake_register!(obj, has_report_link_tls, ss, "report", "link-tls");
    handshake_register!(obj, has_report_tx_begin, ss, "report", "tx-begin");
    handshake_register!(obj, has_report_tx_mail, ss, "report", "tx-mail");
    handshake_register!(obj, has_report_tx_reset, ss, "report", "tx-reset");
    handshake_register!(obj, has_report_tx_rcpt, ss, "report", "tx-rcpt");
    handshake_register!(obj, has_report_tx_envelope, ss, "report", "tx-envelope");
    handshake_register!(obj, has_report_tx_data, ss, "report", "tx-data");
    handshake_register!(obj, has_report_tx_commit, ss, "report", "tx-commit");
    handshake_register!(obj, has_report_tx_rollback, ss, "report", "tx-rollback");
    handshake_register!(
        obj,
        has_report_protocol_client,
        ss,
        "report",
        "protocol-client"
    );
    handshake_register!(
        obj,
        has_report_protocol_server,
        ss,
        "report",
        "protocol-server"
    );
    handshake_register!(
        obj,
        has_report_filter_response,
        ss,
        "report",
        "filter-response"
    );
    handshake_register!(obj, has_report_filter_report, ss, "report", "filter-report");
    handshake_register!(obj, has_report_timeout, ss, "report", "timeout");

    // Ready
    println!("register|ready");
    log::trace!("register ready");
}
