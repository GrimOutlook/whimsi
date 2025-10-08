extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(IntoStrMsiValue)]
pub fn msi_value_convert_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let name = &ast.ident;

    // We assume the existence of an msi::Value type that can be created from a string.
    // The generated code will fail to compile if the struct does not implement ToString.
    let expanded = quote! {
        impl From<#name> for msi::Value {
            fn from(s: #name) -> Self {
                // Convert the struct to a string and then to the msi::Value
                msi::Value::from(s.to_string())
            }
        }
    };

    // Hand back the generated code
    TokenStream::from(expanded)
}
