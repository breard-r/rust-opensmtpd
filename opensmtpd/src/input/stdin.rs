// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::entry::Entry;
use crate::errors::Error;
use crate::input::FilterInput;
use std::default::Default;
use std::io::{self, ErrorKind, Read};
use std::str;

const BUFFER_SIZE: usize = 4096;

pub struct StdIn {
    buffer: [u8; BUFFER_SIZE],
    stdin: io::Stdin,
    input: String,
}

impl Default for StdIn {
    fn default() -> Self {
        StdIn {
            buffer: [0; BUFFER_SIZE],
            stdin: io::stdin(),
            input: String::new(),
        }
    }
}

impl FilterInput for StdIn {
    fn next(&mut self) -> Result<Entry, Error> {
        let mut force_read = false;
        loop {
            if force_read || self.input.is_empty() {
                // Reset the flag
                force_read = false;
                // Read stdin in self.buffer
                self.buffer.copy_from_slice(&[0; BUFFER_SIZE]);
                let len = match self.stdin.read(&mut self.buffer) {
                    Ok(n) => n,
                    Err(e) => match e.kind() {
                        ErrorKind::Interrupted => {
                            continue;
                        }
                        _ => {
                            return Err(e.into());
                        }
                    },
                };
                if len == 0 {
                    return Err(Error::new("Unable to read on stdin."));
                }
                // Put the buffer's content in self.input
                self.input += match self.buffer.iter().position(|&x| x == 0) {
                    Some(i) => str::from_utf8(&self.buffer[..i]),
                    None => str::from_utf8(&self.buffer),
                }?;
            }
            // Try to build an entry from self.input
            let (remainder, entry_opt) = Entry::new(&self.input)?;
            match entry_opt {
                // We have at least one entry.
                Some(entry) => {
                    self.input = remainder;
                    return Ok(entry);
                }
                // The data is incomplete, no entry could be built.
                None => {
                    force_read = true;
                }
            };
        }
    }
}
