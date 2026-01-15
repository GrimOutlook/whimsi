use pretty_assertions::assert_eq;
use quote::ToTokens;
use quote::quote;

use crate::msi_tables;

#[test]
fn test_msi_table_without_generated_identifier() {
    let input = quote! {
        #[msi_table(name = "FeatureComponent")]
        struct FeatureComponentDao {
            #[msi_column(primary_key, identifier(foreign_key = "Feature"), category = msi::Category::Identifier, length = 72)]
            feature_: FeatureIdentifier,
            #[msi_column(primary_key, identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
            component_: ComponentIdentifier,
        }
    };

    // Call the macro's internal function
    let output = msi_tables::gen_tables_impl(input);

    let expected_output = quote! {
        use whimsi_lib::types::column::identifier::Identifier;
        use whimsi_lib::types::column::identifier::ToIdentifier;

        #[derive(Clone, Debug, PartialEq, getset::Getters)]
        #[getset(get = "pub")]
        pub struct FeatureComponentDao {
            feature_: FeatureIdentifier,
            component_: ComponentIdentifier,
        }

        impl FeatureComponentDao {
            pub fn new(feature_: impl Into<FeatureIdentifier> ,component_: impl Into <ComponentIdentifier>) -> FeatureComponentDao {
                FeatureComponentDao {
                    feature_: feature_.into(),
                    component_: component_.into()
                }
            }
        }

        impl PrimaryIdentifier for FeatureComponentDao {
            fn primary_identifier(&self) -> Option<Identifier> {
                None
            }
        }

        impl MsiDao for FeatureComponentDao {

            fn conflicts_with(&self, other: &Self) -> bool {
                self.feature_ == other.feature_ && self.component_ == other.component_
            }

            fn to_row(&self) -> Vec<msi::Value> {
                vec![
                    Into::<msi::Value>::into(&self.feature_),
                    Into::<msi::Value>::into(&self.component_),
                ]
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct FeatureComponentTable {
            entries: Vec<FeatureComponentDao>,
        }

        impl MsiTableKind for FeatureComponentTable {
            type TableValue = FeatureComponentDao;

            fn name(&self) -> &'static str {
                "FeatureComponent"
            }

            fn entries(&self) -> &Vec<FeatureComponentDao> {
                &self.entries
            }

            fn entries_mut(&mut self) -> &mut Vec<FeatureComponentDao> {
                &mut self.entries
            }

            fn len(&self) -> usize {
                self.entries.len()
            }

            fn is_empty(&self) -> bool {
                self.len() == 0
            }

            fn primary_key_indices(&self) -> Vec<usize> {
                vec![0usize,1usize,]
            }

            fn columns(&self) -> Vec<msi::Column> {
                vec![
                    msi::Column::build("Feature_").primary_key().foreign_key("Feature", 0).category(msi::Category::Identifier).string(72),
                    msi::Column::build("Component_").primary_key().foreign_key("Component", 0).category(msi::Category::Identifier).string(72),
                ]
            }
        }

    };

    // Compare the generated output with the expected output (e.g., using syn
    // and comparing ASTs)
    let parsed_output = syn::parse2::<syn::File>(output)
        .expect("Failed to parse output of test data");
    let parsed_expected = syn::parse2::<syn::File>(expected_output)
        .expect("Failed to parse reference test data");

    assert_eq!(
        parsed_output.to_token_stream().to_string(),
        parsed_expected.to_token_stream().to_string()
    );
}
