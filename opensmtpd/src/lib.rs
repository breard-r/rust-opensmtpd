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

use crate::entry::{Kind, SessionId, Subsystem};
use log;
use std::collections::{HashMap, HashSet};
use std::default::Default;

pub use crate::errors::Error;
pub use crate::handler::Handler;
pub use crate::logger::SmtpdLogger;
pub use opensmtpd_derive::report;

#[macro_export]
macro_rules! register_contexts {
    ($context: ty) => {
        opensmtpd::register_contexts!($context, $context);
    };
    ($session_context: ty, $filter_context: ty) => {
        type OpenSmtpdFilterContextType = $filter_context;
        type OpenSmtpdSessionContextType = $session_context;
    };
}

#[macro_export]
macro_rules! register_filter_context_only {
    ($context: ty) => {
        type OpenSmtpdFilterContextType = $context;
        type OpenSmtpdSessionContextType = opensmtpd::NoContext;
    };
}

#[macro_export]
macro_rules! register_session_context_only {
    ($context: ty) => {
        type OpenSmtpdFilterContextType = opensmtpd::NoContext;
        type OpenSmtpdSessionContextType = $context;
    };
}

#[macro_export]
macro_rules! register_no_context {
    () => {
        type OpenSmtpdFilterContextType = opensmtpd::NoContext;
        type OpenSmtpdSessionContextType = opensmtpd::NoContext;
    };
}

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
            .collect::<Vec<opensmtpd::Handler<$sesion_ctx, $filter_ctx>>>();
        let _ = opensmtpd::SmtpdLogger::new().set_level($log_level).init();
        opensmtpd::Filter::<
            opensmtpd::input::StdIn,
            opensmtpd::output::StdOut,
            $sesion_ctx,
            $filter_ctx,
        >::default()
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

#[derive(Clone, Default)]
pub struct NoContext;

pub struct Filter<I, O, S, F>
where
    I: crate::input::FilterInput + Default,
    O: crate::output::FilterOutput + Default,
    S: Default,
    F: Default,
{
    input: I,
    output: O,
    session_ctx: HashMap<SessionId, S>,
    filter_ctx: F,
    handlers: Vec<Handler<S, F>>,
}

impl<I, O, S, F> Default for Filter<I, O, S, F>
where
    I: crate::input::FilterInput + Default,
    O: crate::output::FilterOutput + Default,
    S: Default,
    F: Default,
{
    fn default() -> Self {
        Filter {
            input: I::default(),
            output: O::default(),
            session_ctx: HashMap::new(),
            filter_ctx: F::default(),
            handlers: Vec::new(),
        }
    }
}

impl<I, O, S, F> Filter<I, O, S, F>
where
    I: crate::input::FilterInput + Default,
    O: crate::output::FilterOutput + Default,
    S: Clone + Default,
    F: Clone + Default,
{
    pub fn set_handlers(&mut self, handlers: &[Handler<S, F>]) -> &mut Self {
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
                    let session_id = entry.get_session_id();
                    let mut session_ctx = match self.session_ctx.get_mut(&session_id) {
                        Some(c) => c,
                        None => {
                            self.session_ctx.insert(session_id, S::default());
                            self.session_ctx.get_mut(&session_id).unwrap()
                        }
                    };
                    for h in self.handlers.iter() {
                        match h.send(
                            &entry,
                            &mut self.output,
                            &mut session_ctx,
                            &mut self.filter_ctx,
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                log::warn!("Warning: {}", e);
                            }
                        };
                    }
                    if entry.is_disconnect() {
                        self.session_ctx.remove(&session_id);
                    }
                }
                Err(e) => {
                    fatal_error!(e);
                }
            };
        }
    }
}
