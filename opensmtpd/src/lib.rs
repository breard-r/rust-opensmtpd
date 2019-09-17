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

use log;
use std::default::Default;

pub use crate::errors::Error;
pub use crate::handler::Handler;
pub use crate::logger::SmtpdLogger;
pub use opensmtpd_derive::report;

#[macro_export]
macro_rules! simple_filter {
    ($( $x:expr ),*) => {
        opensmtpd::simple_filter_log_level!(log::Level::Info $(,$x)*);
    };
}

#[macro_export]
macro_rules! simple_filter_log_level {
    ($log_level: expr, $( $x:expr ),*) => {
        let mut handlers = Vec::new();
        $(
            handlers.push(($x)());
        )*;
        let _ = opensmtpd::SmtpdLogger::new()
            .set_level($log_level)
            .init();
        opensmtpd::Filter::<opensmtpd::input::StdIn, opensmtpd::output::StdOut>::default().set_handlers(&handlers).register_events().run();
    };
}

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
        // TODO: use self.output to register events
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
                    log::error!("Error: {}", e);
                    std::process::exit(1);
                }
            };
        }
    }
}
