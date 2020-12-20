use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn register(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let fn_name = item.sig.ident.to_string().replacen("on_", "has_", 1);
    let fn_name = Ident::new(&fn_name, Span::call_site());
    let output = quote! {
        fn #fn_name(&self) -> bool {
            true
        }
        #item
    };
    output.into()
}
