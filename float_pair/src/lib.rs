use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

trait TupleFloat {
    fn generate
}

#[proc_macro_derive(TupleFloat)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl TupleFloat for #ident {}
    };
    output.into()
}
