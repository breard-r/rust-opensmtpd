use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FilterKind {
    Builtin,
    Proc,
}

impl ToString for FilterKind {
    fn to_string(&self) -> String {
        match self {
            FilterKind::Builtin => String::from("builtin"),
            FilterKind::Proc => String::from("proc"),
        }
    }
}

impl FromStr for FilterKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "builtin" => Ok(FilterKind::Builtin),
            "proc" => Ok(FilterKind::Proc),
            _ => Err(()),
        }
    }
}
