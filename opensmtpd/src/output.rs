// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::errors::Error;

pub trait FilterOutput {
    fn send(&mut self, msg: &str) -> Result<(), Error>;
}

mod null;
mod stdout;

pub use null::NullOutput;
pub use stdout::{StdErr, StdOut};
