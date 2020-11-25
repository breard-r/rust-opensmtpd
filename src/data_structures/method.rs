use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Method {
    Helo,
    Ehlo,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Helo => String::from("HELO"),
            Method::Ehlo => String::from("EHLO"),
        }
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HELO" => Ok(Method::Helo),
            "EHLO" => Ok(Method::Ehlo),
            _ => Err(()),
        }
    }
}
