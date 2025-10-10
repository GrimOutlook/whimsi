extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

#[proc_macro_derive(StrToValue)]
pub fn str_to_value(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    make_trait_impl(name, quote! { self.to_string().into() })
}

#[proc_macro_derive(IdentifierToValue)]
pub fn identifier_to_value(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    make_trait_impl(name, quote! { self.to_identifier().to_string().into() })
}

#[proc_macro_derive(IntToValue)]
pub fn int_to_value(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    make_trait_impl(name, quote! { Into::<i32>::into(self.clone()).into() })
}

#[proc_macro_derive(WrapperToValue)]
pub fn wrapper_to_value(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    make_trait_impl(name, quote! { self.0.into() })
}

#[proc_macro_derive(BitmaskToValue)]
pub fn bitmask_to_value_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    make_trait_impl(name, quote! { self.bits().into() })
}

fn make_trait_impl(
    name: &syn::Ident,
    fn_body: proc_macro2::TokenStream,
) -> proc_macro::TokenStream {
    quote! {
        impl msi::ToValue for #name {
            fn to_value(&self) -> msi::Value {
                #fn_body
            }
        }
    }
    .into()
}
