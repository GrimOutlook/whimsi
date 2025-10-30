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

#[proc_macro_derive(ReprToValue)]
pub fn repr_to_value_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    make_trait_impl(name, quote! { (self.clone() as i32).into() })
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

// NOTE: Objectives that this macro must handle:
// - Create a valid Table object from the given inputs.
//     - Determine what the stored datatypes are.
//     - Determine what datatype the stored data must be converted into for
//       insertion.
//         - Just make From<T> to Value a constraint on stored datatypes and we
//           can just use
//         `.into()`.
//     - Determine which columns are primary keys.
//     - Determine which columns are nullable.
//         - Wrap in `Option`?
//     - Determine which columns are generated on the fly and which need to be
//       accepted in the
//     constructor.
//     - Determine which columns must have unique values for each row in the
//       table.
//     - Determine which columns must be unique accross the MSI.
//     - Allow custom implementations for insertion into certain tables.

pub(crate) mod constants;
pub(crate) mod dao;
pub(crate) mod helper;
pub(crate) mod identifier;
mod msi_tables;
pub(crate) mod table;

#[proc_macro]
pub fn msi_table_list(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    msi_tables::gen_tables_impl(input).into()
}

// The `gen_tables_impl` function handles both enums and structs so we can call
// it for both single table generation and multi table generation.
#[proc_macro]
pub fn msi_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    msi_table_list(input)
}
