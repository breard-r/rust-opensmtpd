// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, Ident, Result, Token};

#[derive(Debug)]
pub(crate) struct OpenSmtpdAttributes {
    version: Ident,
    subsystem: Ident,
    events: Punctuated<Ident, Token![,]>,
}

impl Parse for OpenSmtpdAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let version = input.parse()?;
        let _: Token![,] = input.parse()?;
        let subsystem = input.parse()?;
        let _: Token![,] = input.parse()?;
        let _match: Token![match] = input.parse()?;
        let content;
        let _ = parenthesized!(content in input);
        let events = content.parse_terminated(Ident::parse)?;
        Ok(OpenSmtpdAttributes {
            version,
            subsystem,
            events,
        })
    }
}

impl OpenSmtpdAttributes {
    pub(crate) fn get_version(&self) -> String {
        format!(
            "opensmtpd::entry::Version::{}",
            self.version.to_string().to_uppercase()
        )
    }

    pub(crate) fn get_subsystem(&self) -> String {
        let subsystem = match self.subsystem.to_string().as_str() {
            "smtp_in" => "SmtpIn",
            "smtp_out" => "SmtpOut",
            _ => "",
        };
        format!("opensmtpd::entry::Subsystem::{}", subsystem)
    }

    pub(crate) fn get_events(&self) -> String {
        let events = if self
            .events
            .iter()
            .any(|e| e.to_string().to_lowercase().as_str() == "all")
        {
            let lst = [
                "LinkAuth",
                "LinkConnect",
                "LinkDisconnect",
                "LinkIdentify",
                "LinkReset",
                "LinkTls",
                "TxBegin",
                "TxMail",
                "TxRcpt",
                "TxEnvelope",
                "TxData",
                "TxCommit",
                "TxRollback",
                "ProtocolClient",
                "ProtocolServer",
                "FilterResponse",
                "Timeout",
            ];
            lst.iter()
                .map(|e| format!("opensmtpd::entry::Event::{}", e))
                .collect::<Vec<String>>()
        } else {
            self.events
                .iter()
                .map(|e| {
                    let name = match e.to_string().as_str() {
                        "link_auth" => "LinkAuth",
                        "link_connect" => "LinkConnect",
                        "link_disconnect" => "LinkDisconnect",
                        "link_identify" => "LinkIdentify",
                        "link_reset" => "LinkReset",
                        "link_tls" => "LinkTls",
                        "tx_begin" => "TxBegin",
                        "tx_mail" => "TxMail",
                        "tx_rcpt" => "TxRcpt",
                        "tx_envelope" => "TxEnvelope",
                        "tx_data" => "TxData",
                        "tx_commit" => "TxCommit",
                        "tx_rollback" => "TxRollback",
                        "protocol_client" => "ProtocolClient",
                        "protocol_server" => "ProtocolServer",
                        "filter_response" => "FilterResponse",
                        "timeout" => "Timeout",
                        _ => "",
                    };
                    format!("opensmtpd::entry::Event::{}", name)
                })
                .collect::<Vec<String>>()
        };
        format!("[{}]", events.join(", "))
    }
}
