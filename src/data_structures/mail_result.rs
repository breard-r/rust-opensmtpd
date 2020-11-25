use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MailResult {
    Ok,
    PermFail,
    TempFail,
}

impl ToString for MailResult {
    fn to_string(&self) -> String {
        match self {
            MailResult::Ok => String::from("ok"),
            MailResult::PermFail => String::from("permfail"),
            MailResult::TempFail => String::from("tempfail"),
        }
    }
}

impl FromStr for MailResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ok" => Ok(MailResult::Ok),
            "permfail" => Ok(MailResult::PermFail),
            "tempfail" => Ok(MailResult::TempFail),
            _ => Err(()),
        }
    }
}
