use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthResult {
    Pass,
    Fail,
    Error,
}

impl ToString for AuthResult {
    fn to_string(&self) -> String {
        match self {
            AuthResult::Pass => String::from("pass"),
            AuthResult::Fail => String::from("fail"),
            AuthResult::Error => String::from("error"),
        }
    }
}

impl FromStr for AuthResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pass" => Ok(AuthResult::Pass),
            "fail" => Ok(AuthResult::Fail),
            "error" => Ok(AuthResult::Error),
            _ => Err(()),
        }
    }
}
