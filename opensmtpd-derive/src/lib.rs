// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse_macro_input, ExprArray, Ident, ItemFn, Result, Token, TypePath};

#[derive(Debug)]
struct OpenSmtpdAttributes {
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
    fn get_version(&self) -> String {
        format!(
            "opensmtpd::entry::Version::{}",
            self.version.to_string().to_uppercase()
        )
    }

    fn get_subsystem(&self) -> String {
        let subsystem = match self.subsystem.to_string().as_str() {
            "smtp_in" => "SmtpIn",
            "smtp_out" => "SmtpOut",
            _ => "",
        };
        format!("opensmtpd::entry::Subsystem::{}", subsystem)
    }

    fn get_events(&self) -> String {
        let events = if self
            .events
            .iter()
            .find(|&e| e.to_string().to_lowercase().as_str() == "all")
            .is_some()
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

macro_rules! parse_item {
    ($item: expr, $type: ty) => {
        match syn::parse_str::<$type>($item) {
            Ok(i) => i,
            Err(e) => {
                return TokenStream::from(e.to_compile_error());
            }
        }
    };
}

#[proc_macro_attribute]
pub fn report(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as OpenSmtpdAttributes);
    let version = parse_item!(&attr.get_version(), TypePath);
    let subsystem = parse_item!(&attr.get_subsystem(), TypePath);
    let events = parse_item!(&attr.get_events(), ExprArray);
    let item = parse_macro_input!(input as ItemFn);
    let fn_name = &item.sig.ident;
    let fn_params = &item.sig.inputs;
    let fn_body = &item.block;
    let output = quote! {
        fn #fn_name() -> opensmtpd::Handler {
            opensmtpd::Handler::new(
                #version,
                opensmtpd::entry::Kind::Report,
                #subsystem,
                &#events,
                |_output: &mut dyn opensmtpd::output::FilterOutput, entry: &opensmtpd::entry::Entry,| {
                    // TODO: look at `item.sig.output` and adapt the calling scheme.
                    // example: if no return, add `Ok(())`.
                    // https://docs.rs/syn/1.0.5/syn/struct.Signature.html
                    let inner_fn = |#fn_params| -> Result<(), String> {
                        #fn_body
                    };
                    inner_fn(entry)
                },
                )
        }
    };
    output.into()
}
