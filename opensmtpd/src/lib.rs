// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod errors;
mod handler;
mod logger;

pub mod entry;
pub mod input;
pub mod output;

use crate::entry::{Kind, Subsystem};
use log;
use std::collections::HashSet;
use std::default::Default;

pub use crate::errors::Error;
pub use crate::handler::Handler;
pub use crate::logger::SmtpdLogger;
pub use opensmtpd_derive::report;

#[macro_export]
macro_rules! simple_filter {
    ($handlers: expr) => {
        opensmtpd::simple_filter!(
            log::Level::Info,
            opensmtpd::NoContext,
            opensmtpd::NoContext,
            $handlers
        );
    };
    ($filter_ctx: ty, $handlers: expr) => {
        opensmtpd::simple_filter!(
            log::Level::Info,
            opensmtpd::NoContext,
            $filter_ctx,
            $handlers
        );
    };
    ($sesion_ctx: ty, $filter_ctx: ty, $handlers: expr) => {
        opensmtpd::simple_filter!(log::Level::Info, $sesion_ctx, $filter_ctx, $handlers);
    };
    ($log_level: path, $sesion_ctx: ty, $filter_ctx: ty, $handlers: expr) => {
        let handlers = ($handlers)
            .iter()
            .map(|f| f())
            .collect::<Vec<opensmtpd::Handler>>();
        let _ = opensmtpd::SmtpdLogger::new().set_level($log_level).init();
        opensmtpd::Filter::<opensmtpd::input::StdIn, opensmtpd::output::StdOut>::default()
            .set_handlers(handlers.as_slice())
            .register_events()
            .run();
    };
}

macro_rules! fatal_error {
    ($error: ident) => {
        log::error!("Error: {}", $error);
        std::process::exit(1);
    };
}

macro_rules! insert_events {
    ($handler: ident, $set: ident) => {{
        for e in $handler.events.iter() {
            $set.insert(e);
        }
    }};
}

macro_rules! register_events {
    ($output: expr, $set: ident, $kind: expr, $subsystem: expr) => {
        for e in $set.iter() {
            let msg = format!("register|{}|{}|{}", $kind, $subsystem, e.to_string());
            if let Err(e) = $output.send(&msg) {
                fatal_error!(e);
            };
        }
    };
}

#[derive(Default)]
pub struct NoContext;

pub struct Filter<I, O>
where
    I: crate::input::FilterInput + Default,
    O: crate::output::FilterOutput + Default,
{
    input: I,
    output: O,
    handlers: Vec<Handler>,
}

impl<I, O> Default for Filter<I, O>
where
    I: crate::input::FilterInput + Default,
    O: crate::output::FilterOutput + Default,
{
    fn default() -> Self {
        Filter {
            input: I::default(),
            output: O::default(),
            handlers: Vec::new(),
        }
    }
}

impl<I, O> Filter<I, O>
where
    I: crate::input::FilterInput + Default,
    O: crate::output::FilterOutput + Default,
{
    pub fn set_handlers(&mut self, handlers: &[Handler]) -> &mut Self {
        self.handlers = handlers.to_vec();
        self
    }

    pub fn register_events(&mut self) -> &mut Self {
        let mut report_smtp_in = HashSet::new();
        let mut report_smtp_out = HashSet::new();
        let mut filter_smtp_in = HashSet::new();
        let mut filter_smtp_out = HashSet::new();
        for h in self.handlers.iter() {
            match h.kind {
                Kind::Report => match h.subsystem {
                    Subsystem::SmtpIn => insert_events!(h, report_smtp_in),
                    Subsystem::SmtpOut => insert_events!(h, report_smtp_out),
                },
                Kind::Filter => match h.subsystem {
                    Subsystem::SmtpIn => insert_events!(h, filter_smtp_in),
                    Subsystem::SmtpOut => insert_events!(h, filter_smtp_out),
                },
            };
        }
        register_events!(self.output, report_smtp_in, "report", "smtp-in");
        register_events!(self.output, report_smtp_out, "report", "smtp-out");
        register_events!(self.output, filter_smtp_in, "filter", "smtp-in");
        register_events!(self.output, filter_smtp_out, "filter", "smtp-out");
        self
    }

    pub fn run(&mut self) {
        loop {
            match self.input.next() {
                Ok(entry) => {
                    log::debug!("{:?}", entry);
                    for h in self.handlers.iter() {
                        match h.send(&entry, &mut self.output) {
                            Ok(_) => {}
                            Err(e) => {
                                log::warn!("Warning: {}", e);
                            }
                        };
                    }
                }
                Err(e) => {
                    fatal_error!(e);
                }
            };
        }
    }
}
