use pretty_assertions::assert_eq;
use quote::ToTokens;
use quote::quote;

use crate::msi_tables;

#[test]
fn test_msi_tables_enum() {
    let input = quote! {
        enum MsiTable {
            Directory {
                #[msi_column(primary_key, kind(identifier(id_length = "long")))]
                directory: Identifier,
                #[msi_column(kind(identifier(foreign_key(table = "Directory"), id_length = "long")), column_name = "Directory_Parent")]
                parent_directory: Option<Identifier>,
                #[msi_column(kind(string(localizable, category = msi::Category::DefaultDir, length = 255)))]
                default_dir: DefaultDir,
            },

            FeatureComponent {
                #[msi_column(primary_key, kind(identifier(foreign_key(table = "Feature"), id_length = "short")))]
                feature_: Identifier,
                #[msi_column(primary_key, kind(identifier(foreign_key(table = "Component"), id_length = "long")))]
                component_: Identifier,
            }
        }
    };

    // Call the macro's internal function
    let output = msi_tables::gen_tables_impl(input);

    let expected_output = quote! {
        use whimsi_lib::types::column::identifier::Identifier;

        #[derive(Clone, PartialEq, strum::EnumDiscriminants, derive_more::From, derive_more::TryFrom, derive_more::TryInto, strum::Display)]
        #[strum_discriminants(name(MsiTableKind))]
        pub enum MsiTable {
            Directory(DirectoryTable),
            FeatureComponent(FeatureComponentTable),
        }

        #[derive(Clone, PartialEq)]
        pub enum MsiTableDao {
            Directory(DirectoryDao),
            FeatureComponent(FeatureComponentDao),
        }

        #[derive(Clone, Debug, PartialEq, getset::Getters)]
        #[getset(get = "pub")]
        pub struct DirectoryDao {
            directory: Identifier,
            parent_directory: Option<Identifier>,
            default_dir: DefaultDir,
        }

        impl DirectoryDao {
            pub fn new(directory: impl Into<&Identifier>, parent_directory: impl Into<&Option<Identifier>>, default_dir: impl Into<&DefaultDir>) -> DirectoryDao {
                DirectoryDao {
                    directory: directory.into().clone(),
                    parent_directory: parent_directory.into().clone(),
                    default_dir: default_dir.into().clone()
                }
            }
        }

        impl PrimaryIdentifier for DirectoryDao {
            fn primary_identifier(&self) -> Option<Identifier> {
                Some( self.directory.clone() )
            }
        }

        impl MsiDao for DirectoryDao {

            fn conflicts_with(&self, other: &Self) -> bool {
                self.directory == other.directory
            }

            fn to_row(&self) -> Vec<msi::Value> {
                vec![
                    IntoMsiValue::into(&self.directory),
                    IntoMsiValue::into(&self.parent_directory),
                    IntoMsiValue::into(&self.default_dir),
                ]
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct DirectoryTable {
            entries: Vec<DirectoryDao>,
        }

        impl PackageTable for DirectoryTable {

            fn name(&self) -> &'static str {
                "Directory"
            }

            fn primary_key_indices(&self) -> Vec<usize> {
                vec![0usize,]
            }

            fn columns(&self) -> Vec<msi::Column> {
                vec![
                    msi::Column::build("Directory").primary_key().id_string(72usize),
                    msi::Column::build("Directory_Parent").nullable().foreign_key("Directory", 0).id_string(72usize),
                    msi::Column::build("DefaultDir").localizable().category(msi::Category::DefaultDir).string(255),
                ]
            }
        }

        impl DaoList for DirectoryTable {
            type Dao = DirectoryDao;

            fn entries(&self) -> &Vec<DirectoryDao> {
                &self.entries
            }

            fn entries_mut(&mut self) -> &mut Vec<DirectoryDao> {
                &mut self.entries
            }
        }

        #[derive(Clone, Debug, PartialEq, getset::Getters)]
        #[getset(get = "pub")]
        pub struct FeatureComponentDao {
            feature_: Identifier,
            component_: Identifier,
        }

        impl FeatureComponentDao {
            pub fn new(feature_: impl Into<&Identifier> ,component_: impl Into <&Identifier>) -> FeatureComponentDao {
                FeatureComponentDao {
                    feature_: feature_.into().clone(),
                    component_: component_.into().clone()
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
                    IntoMsiValue::into(&self.feature_),
                    IntoMsiValue::into(&self.component_),
                ]
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct FeatureComponentTable {
            entries: Vec<FeatureComponentDao>,
        }

        impl PackageTable for FeatureComponentTable {

            fn name(&self) -> &'static str {
                "FeatureComponent"
            }

            fn primary_key_indices(&self) -> Vec<usize> {
                vec![0usize,1usize,]
            }

            fn columns(&self) -> Vec<msi::Column> {
                vec![
                    msi::Column::build("Feature_").primary_key().foreign_key("Feature", 0).id_string(38usize),
                    msi::Column::build("Component_").primary_key().foreign_key("Component", 0).id_string(72usize),
                ]
            }
        }

        impl DaoList for FeatureComponentTable {
            type Dao = FeatureComponentDao;
            fn entries(&self) -> &Vec<FeatureComponentDao> {
                &self.entries
            }

            fn entries_mut(&mut self) -> &mut Vec<FeatureComponentDao> {
                &mut self.entries
            }
        }
    };

    // Compare the generated output with the expected output (e.g., using syn
    // and comparing ASTs)
    let parsed_output = syn::parse2::<syn::File>(output.clone())
        .unwrap_or_else(|_| {
            panic!("Failed to parse output of test data:\n{}", output)
        });
    let parsed_expected = syn::parse2::<syn::File>(expected_output)
        .expect("Failed to parse reference test data");

    assert_eq!(
        parsed_output.to_token_stream().to_string(),
        parsed_expected.to_token_stream().to_string()
    );
}
