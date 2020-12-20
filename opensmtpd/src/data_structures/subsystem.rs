use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubSystem {
    SmtpIn,
}

impl ToString for SubSystem {
    fn to_string(&self) -> String {
        match self {
            SubSystem::SmtpIn => String::from("smtp-in"),
        }
    }
}

impl FromStr for SubSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "smtp-in" => Ok(SubSystem::SmtpIn),
            _ => Err(()),
        }
    }
}
