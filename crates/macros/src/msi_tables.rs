use darling::FromDeriveInput;
use darling::FromField;
use darling::FromMeta;
use darling::FromVariant;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{self};

use crate::dao::generate_dao_tokens;
use crate::helper::*;
use crate::identifier::generate_identifier_tokens;
use crate::table::generate_table_tokens;

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(msi_table))]
pub(crate) struct DeriveInformation {
    pub ident: syn::Ident,
    pub data: darling::ast::Data<VariantInformation, FieldInformation>,

    // WARN: ONLY USED IF DERIVED ITEM IS A `struct`!
    //
    // If this is a struct, the base name of the table to create. EX:
    // "Directory" will produces struct names such as "DirectoryDao" and
    // "DirectoryTable".
    pub name: Option<String>,
}

#[derive(FromVariant, Clone)]
pub(crate) struct VariantInformation {
    pub ident: syn::Ident,
    pub fields: darling::ast::Fields<FieldInformation>,
}

#[derive(FromField, Clone)]
#[darling(attributes(msi_column))]
pub(crate) struct FieldInformation {
    // -- Builtins ------------------------------------------------------------
    // Field name
    pub ident: Option<syn::Ident>,
    // Type of the field
    pub ty: syn::Type,

    // What the name of the column is. If it is not provided the identifier of
    // the field is converted to title case and underscores are removed.
    #[darling(default)]
    pub column_name: Option<String>,

    // Denotes if the given field is an identifier.
    #[darling(rename = "kind")]
    pub field_type: FieldType,

    // Denotes if the given field corresponds to a primary key in the table.
    #[darling(default)]
    pub primary_key: bool,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, darling::FromMeta)]
pub(crate) enum FieldType {
    Identifier(IdentifierOptions),
    String(StringOptions),
    Integer,
    DoubleInteger,
    Binary,
}

#[derive(Clone, Debug, Default, darling::FromMeta)]
pub(crate) struct IdentifierOptions {
    // Identifier length presets. I've only seen 2 lengths for Identifier types
    // so this makes it simpler.
    pub id_length: IdentifierLength,

    // Denotes if the given identifier is a foreign key into the table and if
    // it is, what table the key is from.
    #[darling(default)]
    pub foreign_key: Option<ForeignKey>,
}

#[derive(Clone, Debug, darling::FromMeta)]
pub(crate) struct ForeignKey {
    pub table: String,

    #[darling(default = default_index)]
    pub index: syn::Expr,
}

fn default_index() -> syn::Expr {
    syn::parse_quote!(0)
}

#[derive(Clone, Copy, Debug, Default, darling::FromMeta, strum::FromRepr)]
#[repr(usize)]
pub(crate) enum IdentifierLength {
    Short = 38,
    #[default]
    Long  = 72,
}

#[derive(Clone, Debug, darling::FromMeta)]
pub(crate) struct StringOptions {
    // The kind of string that is to be stored.
    pub category: syn::Expr,

    // The maximum length of the string placed in the column. This is specific
    // to each table so I can't abstract it away. If it is not provided a
    // default based on the provided Category is used.
    //
    // NOTE: I considered making this optional and using sane defaults for
    // columns based on the given category but I like the idea of not
    // obscuring what values are being used for a given column. This is
    // only optional for categories of Integer and DoubleInteger.
    pub length: syn::Expr,

    // Whether or not the given field is localizable as specified in the MSI
    // documentation.
    #[darling(default)]
    pub localizable: bool,
}

pub fn gen_tables_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse2::<syn::DeriveInput>(input).unwrap();
    let derive_input = DeriveInformation::from_derive_input(&input)
        .expect("Failed to parse derive input");

    let output_tokens = match derive_input.data {
        darling::ast::Data::Enum(items) => {
            gen_tables_for_enum(&derive_input.ident.to_string(), items)
        }
        darling::ast::Data::Struct(fields) => {
            let name = capitalize(
                &derive_input.name.unwrap_or(derive_input.ident.to_string()),
            );
            gen_tables_for_fields(&name, fields.fields)
        }
    };

    quote! {
        use whimsi_lib::types::column::identifier::Identifier;
        use whimsi_lib::types::column::identifier::ToIdentifier;

        #output_tokens
    }
}

fn gen_tables_for_enum(
    name: &str,
    items: Vec<VariantInformation>,
) -> TokenStream {
    let (struct_variants, dao_variants) = items
        .iter()
        .map(|v| {
            let variant = v.ident.clone();
            let table_name = table_from_name(&variant.to_string());
            let dao_name = dao_from_name(&variant.to_string());
            (
                quote! { #variant ( #table_name ) , },
                quote! { #variant ( #dao_name ) , },
            )
        })
        .collect::<(Vec<TokenStream>, Vec<TokenStream>)>();

    // Generate the enum containing all of the variant structs
    let table_enum_name = format_ident!("{name}");
    let table_variant_enum_name = format_ident!("{name}Kind");
    let dao_enum_name = dao_from_name(name);
    let tokens = quote! {
        #[derive(Clone, PartialEq, strum::EnumDiscriminants, derive_more::From, derive_more::TryFrom, derive_more::TryInto, strum::Display)]
        #[strum_discriminants(#table_variant_enum_name)]
        pub enum #table_enum_name {
            #(#struct_variants)*
        }

        #[derive(Clone, PartialEq)]
        pub enum #dao_enum_name {
            #(#dao_variants)*
        }
    };
    items.iter().fold(tokens, |acc, variant| {
        let table_def_tokens = gen_tables_for_fields(
            &variant.ident.to_string(),
            variant.fields.fields.clone(),
        );
        quote! {
            #acc
            #table_def_tokens
        }
    })
}

fn gen_tables_for_fields(
    base_name: &str,
    fields: Vec<FieldInformation>,
) -> TokenStream {
    let target_name = capitalize(base_name);

    // Create the table-specific identifier if one should be made. These are
    // made when a table has a column with a type that has a field_type of
    // Identifier and the column is not marked as storing a foreign key.
    let primary_identifier = fields
        .iter()
        .filter(|field| {
            if field.primary_key && let FieldType::Identifier(options) = &field.field_type && options.foreign_key.is_none() {
                true
            } else {
                false
            }
        })
        .at_most_one()
        .unwrap_or_else(|_| {
            panic!(
                "More than one field marked as primary identifier found in definition. This is not supported."
            )
        });

    let identifier_tokens = if primary_identifier.is_some() {
        generate_identifier_tokens(&target_name)
    } else {
        Default::default()
    };

    let dao_tokens =
        generate_dao_tokens(&target_name, &primary_identifier, &fields);

    let table_tokens = generate_table_tokens(&target_name, &fields);

    // Generate the DAO code.
    let output_tokens = quote! {
        #identifier_tokens
        #dao_tokens
        #table_tokens
    };

    output_tokens
}

#[cfg(test)]
mod test_enum;
#[cfg(test)]
mod test_table_no_identifier;
#[cfg(test)]
mod test_table_with_identifier;
