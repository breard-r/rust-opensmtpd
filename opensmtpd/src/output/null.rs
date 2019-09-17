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

pub struct NullOutput {}

impl Default for NullOutput {
    fn default() -> Self {
        NullOutput {}
    }
}

impl FilterOutput for NullOutput {
    fn send(&mut self, _msg: &str) -> Result<(), Error> {
        Ok(())
    }
}
