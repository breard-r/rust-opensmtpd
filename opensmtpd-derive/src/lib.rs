// Copyright (c) 2019 Rodolphe Bréard
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate proc_macro;

mod attributes;

use attributes::OpenSmtpdAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprArray, ItemFn, TypePath};

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
