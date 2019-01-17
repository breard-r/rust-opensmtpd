extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

fn get_type(
    params: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> Result<(Box<syn::Type>, syn::Type), ()> {
    match params.iter().count() {
        1 => {
            let ctx = Box::new(syn::Type::Verbatim(syn::TypeVerbatim {
                tts: quote! {
                    opensmtpd::NoContext
                },
            }));
            let cb = syn::Type::Verbatim(syn::TypeVerbatim {
                tts: quote!{ opensmtpd::Callback::NoCtx },
            });
            Ok((ctx, cb))
        }
        2 => match params.iter().next().unwrap() {
            syn::FnArg::Captured(ref a) => match &a.ty {
                syn::Type::Reference(r) => {
                    let cb = match r.mutability {
                        Some(_) => syn::Type::Verbatim(syn::TypeVerbatim {
                            tts: quote!{ opensmtpd::Callback::CtxMut },
                        }),
                        None => syn::Type::Verbatim(syn::TypeVerbatim {
                            tts: quote!{ opensmtpd::Callback::Ctx },
                        }),
                    };
                    Ok((r.elem.clone(), cb))
                }
                _ => Err(()),
            },
            _ => Err(()),
        },
        _ => Err(()),
    }
}

#[proc_macro_attribute]
pub fn event(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let item = parse_macro_input!(input as ItemFn);
    let fn_name = &item.ident;
    let fn_params = &item.decl.inputs;
    let (ctx_type, callback_type) = match get_type(fn_params) {
        Ok(t) => t,
        Err(_) => {
            panic!();
        }
    };
    let fn_body = &item.block;
    let fn_output = &item.decl.output;
    let output = quote! {
        fn #fn_name() -> opensmtpd::EventHandler<#ctx_type> {
            opensmtpd::EventHandler::new(
                #attr,
                #callback_type(|#fn_params| #fn_output #fn_body)
            )
        }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn report(attr: TokenStream, input: TokenStream) -> TokenStream {
    event(attr, input)
}
