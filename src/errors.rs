use crate::entry::Entry;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            message: msg.to_string(),
        }
    }

    pub fn from_string(msg: &String) -> Self {
        Error::new(&msg)
    }

    pub fn new_param(param: &str, msg: &str) -> Self {
        Error::new(&format!("{}: {}", param, msg))
    }

    pub fn as_str(&self) -> &str {
        &self.message
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::from_string(&format!("IO error: {}", error))
    }
}

impl From<nom::Err<&str>> for Error {
    fn from(error: nom::Err<&str>) -> Self {
        let msg = match error {
            nom::Err::Incomplete(_) => "not enough data".to_string(),
            nom::Err::Error(c) => format!("{:?}", c),
            nom::Err::Failure(c) => format!("{:?}", c),
        };
        Error::from_string(&format!("Parsing error: {}", msg))
    }
}

impl From<std::sync::mpsc::SendError<Entry>> for Error {
    fn from(error: std::sync::mpsc::SendError<Entry>) -> Self {
        Error::from_string(&format!("IO error: {}", error))
    }
}
