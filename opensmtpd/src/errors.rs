// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(&format!("IO error: {}", error))
    }
}

impl From<std::string::String> for Error {
    fn from(error: std::string::String) -> Self {
        Error { message: error }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::new(&format!("UTF8 error: {}", error))
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(error: log::SetLoggerError) -> Self {
        Error::new(&format!("Logger error: {}", error))
    }
}

impl From<nom::Err<(&str, nom::error::ErrorKind)>> for Error {
    fn from(error: nom::Err<(&str, nom::error::ErrorKind)>) -> Self {
        let msg = match error {
            nom::Err::Incomplete(_) => "not enough data".to_string(),
            nom::Err::Error(c) => format!("{:?}", c),
            nom::Err::Failure(c) => format!("{:?}", c),
        };
        Error::new(&format!("Parsing error: {}", msg))
    }
}
