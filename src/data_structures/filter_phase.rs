use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FilterPhase {
    Connect,
    Helo,
    Ehlo,
    StartTls,
    Auth,
    MailFrom,
    RcptTo,
    Data,
    DataLine,
    Commit,
}

impl ToString for FilterPhase {
    fn to_string(&self) -> String {
        match self {
            FilterPhase::Connect => String::from("connect"),
            FilterPhase::Helo => String::from("helo"),
            FilterPhase::Ehlo => String::from("ehlo"),
            FilterPhase::StartTls => String::from("starttls"),
            FilterPhase::Auth => String::from("auth"),
            FilterPhase::MailFrom => String::from("mail-from"),
            FilterPhase::RcptTo => String::from("rcpt-to"),
            FilterPhase::Data => String::from("data"),
            FilterPhase::DataLine => String::from("data-line"),
            FilterPhase::Commit => String::from("commit"),
        }
    }
}

impl FromStr for FilterPhase {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "connect" => Ok(FilterPhase::Connect),
            "helo" => Ok(FilterPhase::Helo),
            "ehlo" => Ok(FilterPhase::Ehlo),
            "starttls" => Ok(FilterPhase::StartTls),
            "auth" => Ok(FilterPhase::Auth),
            "mail-from" => Ok(FilterPhase::MailFrom),
            "rcpt-to" => Ok(FilterPhase::RcptTo),
            "data" => Ok(FilterPhase::Data),
            "data-line" => Ok(FilterPhase::DataLine),
            "commit" => Ok(FilterPhase::Commit),
            _ => Err(()),
        }
    }
}
