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
use syn::{parse_macro_input, ExprArray, ExprTry, ItemFn, ReturnType, TypePath};

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

fn get_has_result(ret: &ReturnType) -> bool {
    match ret {
        ReturnType::Default => false,
        ReturnType::Type(_, _) => true,
    }
}

fn get_inner_call(nb_args: usize, has_output: bool, has_result: bool) -> String {
    let mut call_params = Vec::new();
    if has_output {
        call_params.push("_output");
    }
    call_params.push("_entry");
    if nb_args >= 2 {
        call_params.push("_filter_ctx");
    }
    if nb_args >= 3 {
        call_params.push("_session_ctx");
    }
    let call_params = call_params.join(", ");
    let s = format!("inner_fn({})", &call_params);
    if has_result {
        return format!("{}?", s);
    }
    format!(
        "(|{params}| -> Result<(), String> {{ {inner_fn}; Ok(()) }})({params})?",
        params = &call_params,
        inner_fn = s
    )
}

fn get_tokenstream(
    attr: TokenStream,
    input: TokenStream,
    type_str: &str,
    has_output: bool,
) -> TokenStream {
    let kind = parse_item!(type_str, TypePath);

    // Parse the procedural macro attributes
    let attr = parse_macro_input!(attr as OpenSmtpdAttributes);
    let version = parse_item!(&attr.get_version(), TypePath);
    let subsystem = parse_item!(&attr.get_subsystem(), TypePath);
    let events = parse_item!(&attr.get_events(), ExprArray);

    // Parse the user-supplied function
    let item = parse_macro_input!(input as ItemFn);
    let fn_name = &item.sig.ident;
    let fn_params = &item.sig.inputs;
    let fn_return = &item.sig.output;
    let fn_body = &item.block;
    let has_result = get_has_result(&item.sig.output);
    let inner_call = parse_item!(
        &get_inner_call(fn_params.len(), has_output, has_result),
        ExprTry
    );

    // Build the new function
    let output = quote! {
        fn #fn_name() -> opensmtpd::Handler::<OpenSmtpdSessionContextType, OpenSmtpdFilterContextType> {
            opensmtpd::Handler::new(
                #version,
                #kind,
                #subsystem,
                &#events,
                |_output: &mut dyn opensmtpd::output::FilterOutput, _entry: &opensmtpd::entry::Entry, _session_ctx: &mut OpenSmtpdSessionContextType, _filter_ctx: &mut OpenSmtpdFilterContextType| {
                    let inner_fn = |#fn_params| #fn_return {
                        #fn_body
                    };
                    let _ = #inner_call;
                    Ok(())
                },
            )
        }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn report(attr: TokenStream, input: TokenStream) -> TokenStream {
    get_tokenstream(attr, input, "opensmtpd::entry::Kind::Report", false)
}

#[proc_macro_attribute]
pub fn filter(attr: TokenStream, input: TokenStream) -> TokenStream {
    get_tokenstream(attr, input, "opensmtpd::entry::Kind::Filter", true)
}
