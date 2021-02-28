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
//! ## Reports
//!
//! Reports are very simple: the associated functions accepts a
//! [`ReportEntry`] parameter as well as some optional parameters and
//! do not return anything. The [`ReportEntry`] parameter contains
//! all the common information about reports while the other
//! parameters are specific to each filter.
//!
//! ## Filters
//!
//! Filters are similar to reports: the associated functions accepts
//! a [`FilterEntry`] parameter as well as some optional parameters,
//! but this time they have to return a [`FilterResponse`]. The only
//! exception is the
//! [`on_filter_data_line`](Filter::on_filter_data_line) function
//! that doesn't return anything.
//!
//! ## The data-line filter
//!
//! This filter is the only one that **does not** return a
//! [`FilterResponse`]. Instead, you are expected to produce new
//! lines using the [`return_data_line`] function. This function does
//! not have to be called each time
//! [`on_filter_data_line`](Filter::on_filter_data_line) is
//! triggered: you can store all of the data-lines, edit them and
//! then call [`return_data_line`] on each.
//!
//! The last data-line you will receive is a single dot. The last one
//! you return must also be a single dot.
//!
//! # Examples
//!
//! The following filter increments a variable every time a client
//! disconnects.
//!
//! ``` rust
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
//! The following filter removes the `X-Originating-Ip` header.
//!
//! Be careful, this is not production-ready since it does not
//! support long headers and cannot differentiate a header from
//! the body's content.
//!
//! ``` rust
//! use opensmtpd::{return_data_line, run_filter, Filter, FilterEntry};
//! use opensmtpd_derive::register;
//!
//! pub const HEADER_NAME: &str = "x-originating-ip:";
//! pub const HEADER_LEN: usize = 17;
//!
//! struct RmXOriginatingIp {}
//!
//! impl Filter for RmXOriginatingIp {
//!     #[register]
//!     fn on_filter_data_line(&mut self, entry: &FilterEntry, data_line: &[u8]) {
//!         if data_line.len() >= HEADER_LEN {
//!             let head_start = data_line[..HEADER_LEN].to_vec();
//!             if let Ok(s) = String::from_utf8(head_start) {
//!                 if s.to_lowercase() == HEADER_NAME {
//!                     return;
//!                 }
//!             }
//!         }
//!         return_data_line(entry, data_line);
//!     }
//! }
//!
//! fn main() {
//!     let mut my_filter = RmXOriginatingIp {};
//!     run_filter(&mut my_filter);
//! }
//! ```
//!
//! More examples can be found in the [examples directory](https://github.com/breard-r/rust-opensmtpd/tree/main/opensmtpd/examples).
//!
//! # Documentation about filters
//!
//! This documentation is not meant to provide information about the
//! filters. For that purpose, you should refer to the
//! `smtpd-filters` man page (`man 7 smtpd-filters`).
//!
//! In the case this man page has not been installed with OpenSMTPD
//! or if you want the latest one available, you can download it from
//! the OpenSMTPD repository:
//!
//! ``` sh
//! curl -sSf "https://raw.githubusercontent.com/OpenSMTPD/OpenSMTPD/master/usr.sbin/smtpd/smtpd-filters.7" | man -l -
//! ```
//!
//! Alternatively, using zsh, you can use the following variants.
//! Useful on system where man is unable to read from stdin (yes
//! BSD, that's you).
//!
//! ``` sh
//! man =(curl -sSf "https://raw.githubusercontent.com/OpenSMTPD/OpenSMTPD/master/usr.sbin/smtpd/smtpd-filters.7")
//! ```

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
	handshake_register!(obj, has_filter_auth, ss, "filter", "auth");
	handshake_register!(obj, has_filter_commit, ss, "filter", "commit");
	handshake_register!(obj, has_filter_connect, ss, "filter", "connect");
	handshake_register!(obj, has_filter_data, ss, "filter", "data");
	handshake_register!(obj, has_filter_data_line, ss, "filter", "data-line");
	handshake_register!(obj, has_filter_ehlo, ss, "filter", "ehlo");
	handshake_register!(obj, has_filter_helo, ss, "filter", "helo");
	handshake_register!(obj, has_filter_mail_from, ss, "filter", "mail-from");
	handshake_register!(obj, has_filter_rcpt_to, ss, "filter", "rcpt-to");
	handshake_register!(obj, has_filter_starttls, ss, "filter", "starttls");

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
