extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn event(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let item = parse_macro_input!(input as ItemFn);
    let fn_name = &item.ident;
    let fn_params = &item.decl.inputs;
    let fn_body = &item.block;
    let fn_output = &item.decl.output;
    let output = quote! {
        fn #fn_name() -> opensmtpd::EventHandler {
            opensmtpd::EventHandler::new(#attr.to_string(), |#fn_params| #fn_output #fn_body)
        }
    };
    output.into()
}
