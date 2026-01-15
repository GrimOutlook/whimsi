use pretty_assertions::assert_eq;
use quote::ToTokens;
use quote::quote;

use crate::msi_tables;

#[test]
fn test_msi_table_with_generated_identifier() {
    let input = quote! {
        #[msi_table(name = "Directory")]
        struct Directory {
            #[msi_column(primary_key, kind(identifier(id_length = "long")))]
            directory: DirectoryIdentifier,
            #[msi_column(kind(identifier(foreign_key(table = "Directory"), id_length = "long")), column_name = "Directory_Parent")]
            parent_directory: Option<DirectoryIdentifier>,
            #[msi_column(kind(string(localizable, category = msi::Category::DefaultDir, length = 255)))]
            default_dir: DefaultDir,
        }
    };

    // Call the macro's internal function
    let output = msi_tables::gen_tables_impl(input);

    let expected_output = quote! {
        use whimsi_lib::types::column::identifier::Identifier;

        #[doc = "This is a simple wrapper around `Identifier` for the `DirectoryTable`. Used to ensure that identifiers for the `DirectoryTable` are only used in valid locations."]
        #[derive(Clone, Debug, Default, PartialEq, derive_more::Display, whimsi_macros::IdentifierToValue)]
        pub struct DirectoryIdentifier(Identifier);

        impl From<DirectoryIdentifier> for Identifier {
            fn from(val: DirectoryIdentifier) -> Identifier {
                val.0.clone()
            }
        }

        impl std::str::FromStr for DirectoryIdentifier {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> anyhow::Result<Self> {
                Ok(Self(Identifier::from_str(s)?))
            }
        }

        #[derive(Clone, Debug, PartialEq, getset::Getters)]
        #[getset(get = "pub")]
        pub struct DirectoryDao {
            directory: DirectoryIdentifier,
            parent_directory: Option<DirectoryIdentifier>,
            default_dir: DefaultDir,
        }

        impl DirectoryDao {
            pub fn new(directory: impl Into<DirectoryIdentifier>, parent_directory: impl Into<Option<DirectoryIdentifier>>, default_dir: impl Into<DefaultDir>) -> DirectoryDao {
                DirectoryDao {
                    directory: directory.into(),
                    parent_directory: parent_directory.into(),
                    default_dir: default_dir.into()
                }
            }
        }

        impl PrimaryIdentifier for DirectoryDao {
            fn primary_identifier(&self) -> Option<Identifier> {
                Some( self.directory.into() )
            }
        }

        impl MsiDao for DirectoryDao {

            fn conflicts_with(&self, other: &Self) -> bool {
                self.directory == other.directory
            }

            fn to_row(&self) -> Vec<msi::Value> {
                vec![
                    Into::<msi::Value>::into(self.directory),
                    Into::<msi::Value>::into(self.parent_directory),
                    Into::<msi::Value>::into(self.default_dir),
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
