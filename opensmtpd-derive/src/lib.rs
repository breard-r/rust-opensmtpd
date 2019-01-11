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
        // TODO: set the correct EventHandler type
        fn #fn_name() -> opensmtpd::EventHandler<opensmtpd::NoContext> {
            // TODO: set the correct Callback type
            opensmtpd::EventHandler::new(
                #attr,
                opensmtpd::Callback::CtxMut(|#fn_params| #fn_output #fn_body)
            )
        }
    };
    output.into()
}
