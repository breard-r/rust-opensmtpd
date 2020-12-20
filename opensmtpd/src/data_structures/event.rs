use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    LinkAuth,
    LinkConnect,
    LinkDisconnect,
    LinkGreeting,
    LinkIdentify,
    LinkTls,
    TxBegin,
    TxMail,
    TxReset,
    TxRcpt,
    TxEnvelope,
    TxData,
    TxCommit,
    TxRollback,
    ProtocolClient,
    ProtocolServer,
    FilterResponse,
    FilterReport,
    Timeout,
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            Event::LinkAuth => String::from("link-auth"),
            Event::LinkConnect => String::from("link-connect"),
            Event::LinkDisconnect => String::from("link-disconnect"),
            Event::LinkGreeting => String::from("link-greeting"),
            Event::LinkIdentify => String::from("link-identify"),
            Event::LinkTls => String::from("link-tls"),
            Event::TxBegin => String::from("tx-begin"),
            Event::TxMail => String::from("tx-mail"),
            Event::TxReset => String::from("tx-reset"),
            Event::TxRcpt => String::from("tx-rcpt"),
            Event::TxEnvelope => String::from("tx-envelope"),
            Event::TxData => String::from("tx-data"),
            Event::TxCommit => String::from("tx-commit"),
            Event::TxRollback => String::from("tx-rollback"),
            Event::ProtocolClient => String::from("protocol-client"),
            Event::ProtocolServer => String::from("protocol-server"),
            Event::FilterResponse => String::from("filter-response"),
            Event::FilterReport => String::from("filter-report"),
            Event::Timeout => String::from("timeout"),
        }
    }
}

impl FromStr for Event {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "link-auth" => Ok(Event::LinkAuth),
            "link-connect" => Ok(Event::LinkConnect),
            "link-disconnect" => Ok(Event::LinkDisconnect),
            "link-greeting" => Ok(Event::LinkGreeting),
            "link-identify" => Ok(Event::LinkIdentify),
            "link-tls" => Ok(Event::LinkTls),
            "tx-begin" => Ok(Event::TxBegin),
            "tx-mail" => Ok(Event::TxMail),
            "tx-reset" => Ok(Event::TxReset),
            "tx-rcpt" => Ok(Event::TxRcpt),
            "tx-envelope" => Ok(Event::TxEnvelope),
            "tx-data" => Ok(Event::TxData),
            "tx-commit" => Ok(Event::TxCommit),
            "tx-rollback" => Ok(Event::TxRollback),
            "protocol-client" => Ok(Event::ProtocolClient),
            "protocol-server" => Ok(Event::ProtocolServer),
            "filter-response" => Ok(Event::FilterResponse),
            "filter-report" => Ok(Event::FilterReport),
            "timeout" => Ok(Event::Timeout),
            _ => Err(()),
        }
    }
}
