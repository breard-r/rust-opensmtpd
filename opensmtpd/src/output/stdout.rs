// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::errors::Error;
use crate::output::FilterOutput;
use std::default::Default;

macro_rules! new_stdout {
    ($name: ident, $out: ident) => {
        pub struct $name {}

        impl Default for $name {
            fn default() -> Self {
                $name {}
            }
        }

        impl FilterOutput for $name {
            fn send(&mut self, msg: &str) -> Result<(), Error> {
                $out!("{}", msg);
                Ok(())
            }
        }
    };
}

new_stdout!(StdOut, println);
new_stdout!(StdErr, eprintln);
