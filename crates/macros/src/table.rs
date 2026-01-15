use proc_macro2::TokenStream;
use quote::quote;

use crate::helper::*;
use crate::msi_tables::FieldInformation;
use crate::msi_tables::FieldType;

pub fn generate_table_tokens(
    target_name: &str,
    fields: &[FieldInformation],
) -> TokenStream {
    let table_definition_tokens = generate_table_definition(target_name);
    let msi_table_impl_tokens = generate_msi_table_impl(target_name, fields);
    quote! {
        #table_definition_tokens
        #msi_table_impl_tokens
    }
}

fn generate_table_definition(target_name: &str) -> TokenStream {
    let table_ident = table_from_name(target_name);
    let dao_type = dao_from_name(target_name);

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub struct #table_ident {
            entries: Vec<#dao_type>,
        }
    }
}

fn generate_msi_table_impl(
    target_name: &str,
    fields: &[FieldInformation],
) -> TokenStream {
    let primary_key_indices =
        fields.iter().enumerate().fold(quote! {}, |acc, (index, field)| {
            if field.primary_key {
                quote! { #acc #index, }
            } else {
                acc
            }
        });

    let columns = fields.iter().fold(quote! {}, |acc, field| {
        let field_ident = &field.ident.clone().expect("Field doesn't have an identifier");

        let column_name = if let Some(column_name) = &field.column_name {column_name} else { &snake_case_to_pascal_case(&field_ident.to_string())};
        let nullable = if let syn::Type::Path(path) = &field.ty &&
                 path.path.segments.last().unwrap().ident == "Option" {
                    quote!{.nullable()}
                }
            else {
                Default::default()
            };

        let primary_key = if field.primary_key {
            quote!{.primary_key()}
        } else {
            Default::default()
        };

        // If this causes issues it can probably be removed.
        let foreign_key = if let FieldType::Identifier(options) = &field.field_type &&
            let Some(foreign_key) = &options.foreign_key {
            let table = &foreign_key.table;
            let index = &foreign_key.index;
            quote!{.foreign_key(#table, #index)}
        } else {
            Default::default()
        };

        let finish = match &field.field_type {
            FieldType::Identifier(options) => {
                let length = options.id_length as usize;
                quote!{ .id_string( #length ) }
            },
            FieldType::String(options) => {
                let field_category = &options.category;
                let string_length = &options.length;
                let localizable = match &options.localizable {
                    true => quote!(.localizable()),
                    false => Default::default(),
                };
                quote! { #localizable .category( #field_category ).string( #string_length ) }
            },
            FieldType::Integer => quote! { .int16() },
            FieldType::DoubleInteger => quote! { .int32() },
            FieldType::Binary => quote! { .binary() },
        };

        quote! {
            #acc

            msi::Column::build(#column_name) #primary_key #nullable #foreign_key #finish,
        }
    });

    let table_name = table_from_name(target_name);
    let dao_name = dao_from_name(target_name);

    quote! {
        impl PackageTable for #table_name {

            fn name(&self) -> &'static str {
                #target_name
            }

            fn primary_key_indices(&self) -> Vec<usize> {
                vec![#primary_key_indices]
            }

            fn columns(&self) -> Vec<msi::Column> {
                vec![
                    #columns
                ]
            }
        }

        impl DaoList for #table_name {
            type Dao = #dao_name;

            fn entries(&self) -> &Vec<#dao_name> {
                &self.entries
            }

            fn entries_mut(&mut self) -> &mut Vec<#dao_name> {
                &mut self.entries
            }
        }
    }
}
