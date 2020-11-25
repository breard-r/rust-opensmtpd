use crate::SmtpStatusCode;

#[derive(Clone, Debug)]
pub enum FilterResponse {
    Proceed,
    Junk,
    Reject(SmtpStatusCode),
    Disconnect(SmtpStatusCode),
    Rewrite(String),
    Report(String),
}

impl ToString for FilterResponse {
    fn to_string(&self) -> String {
        match self {
            FilterResponse::Proceed => String::from("proceed"),
            FilterResponse::Junk => String::from("junk"),
            FilterResponse::Reject(e) => format!("reject|{}", e.to_string()),
            FilterResponse::Disconnect(e) => format!("disconnect|{}", e.to_string()),
            FilterResponse::Rewrite(s) => format!("rewrite|{}", s),
            FilterResponse::Report(s) => format!("report|{}", s),
        }
    }
}
