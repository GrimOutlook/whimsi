use pretty_assertions::assert_eq;
use quote::ToTokens;
use quote::quote;

use crate::msi_tables;

#[test]
fn test_msi_table_with_generated_identifier() {
    let input = quote! {
        #[msi_table(name = "Directory")]
        struct Directory {
            #[msi_column(primary_key, identifier(), category = msi::Category::Identifier, length = 72)]
            directory: DirectoryIdentifier,
            #[msi_column(identifier(foreign_key = "Directory"), column_name = "Directory_Parent", category = msi::Category::Identifier, length = 72)]
            parent_directory: Option<DirectoryIdentifier>,
            #[msi_column(localizable, category = msi::Category::DefaultDir, length = 255)]
            default_dir: DefaultDir,
        }
    };

    // Call the macro's internal function
    let output = msi_tables::gen_tables_impl(input);

    let expected_output = quote! {
        use whimsi_lib::types::column::identifier::Identifier;
        use whimsi_lib::types::column::identifier::ToIdentifier;

        #[doc = "This is a simple wrapper around `Identifier` for the `DirectoryTable`. Used to ensure that identifiers for the `DirectoryTable` are only used in valid locations."]
        #[derive(Clone, Debug, Default, PartialEq, derive_more::Display, whimsi_macros::IdentifierToValue)]
        pub struct DirectoryIdentifier(Identifier);

        impl ToIdentifier for DirectoryIdentifier {
            fn to_identifier(&self) -> Identifier {
                self.0.clone()
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
                Some( self.directory.to_identifier() )
            }
        }

        impl MsiDao for DirectoryDao {

            fn conflicts_with(&self, other: &Self) -> bool {
                self.directory == other.directory
            }

            fn to_row(&self) -> Vec<msi::Value> {
                vec![
                    msi::ToValue::to_value(&self.directory),
                    msi::ToValue::to_value(&self.parent_directory),
                    msi::ToValue::to_value(&self.default_dir),
                ]
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct DirectoryTable {
            entries: Vec<DirectoryDao>,
        }

        impl MsiTableKind for DirectoryTable {
            type TableValue = DirectoryDao;

            fn name(&self) -> &'static str {
                "Directory"
            }

            fn entries(&self) -> &Vec<DirectoryDao> {
                &self.entries
            }

            fn entries_mut(&mut self) -> &mut Vec<DirectoryDao> {
                &mut self.entries
            }

            fn len(&self) -> usize {
                self.entries.len()
            }

            fn is_empty(&self) -> bool {
                self.len() == 0
            }

            fn primary_key_indices(&self) -> Vec<usize> {
                vec![0usize,]
            }

            fn columns(&self) -> Vec<msi::Column> {
                vec![
                    msi::Column::build("Directory").primary_key().category(msi::Category::Identifier).string(72),
                    msi::Column::build("Directory_Parent").nullable().foreign_key("Directory", 0).category(msi::Category::Identifier).string(72),
                    msi::Column::build("DefaultDir").localizable().category(msi::Category::DefaultDir).string(255),
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
                    msi::ToValue::to_value(&self.feature_),
                    msi::ToValue::to_value(&self.component_),
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

#[test]
fn test_msi_tables_enum() {
    let input = quote! {
        enum MsiTables {
            Directory {
                #[msi_column(primary_key, identifier(), category = msi::Category::Identifier, length = 72)]
                directory: DirectoryIdentifier,
                #[msi_column(identifier(foreign_key = "Directory"), column_name = "Directory_Parent", category = msi::Category::Identifier, length = 72)]
                parent_directory: Option<DirectoryIdentifier>,
                #[msi_column(localizable, category = msi::Category::DefaultDir, length = 255)]
                default_dir: DefaultDir,
            },

            FeatureComponent {
                #[msi_column(primary_key, identifier(foreign_key = "Feature"), category = msi::Category::Identifier, length = 72)]
                feature_: FeatureIdentifier,
                #[msi_column(primary_key, identifier(foreign_key = "Component"), category = msi::Category::Identifier, length = 72)]
                component_: ComponentIdentifier,
            }
        }
    };

    // Call the macro's internal function
    let output = msi_tables::gen_tables_impl(input);

    let expected_output = quote! {
        use whimsi_lib::types::column::identifier::Identifier;
        use whimsi_lib::types::column::identifier::ToIdentifier;

        #[derive(Clone, PartialEq, strum::EnumDiscriminants, derive_more::From, derive_more::TryFrom, derive_more::TryInto, strum::Display)]
        #[strum_discriminants(name(MsiTable))]
        pub enum MsiTables {
            Directory(DirectoryTable),
            FeatureComponent(FeatureComponentTable),
        }

        #[derive(Clone, PartialEq)]
        pub enum MsiTablesDao {
            Directory(DirectoryDao),
            FeatureComponent(FeatureComponentDao),
        }

        #[doc = "This is a simple wrapper around `Identifier` for the `DirectoryTable`. Used to ensure that identifiers for the `DirectoryTable` are only used in valid locations."]
        #[derive(Clone, Debug, Default, PartialEq, derive_more::Display, whimsi_macros::IdentifierToValue)]
        pub struct DirectoryIdentifier(Identifier);

        impl ToIdentifier for DirectoryIdentifier {
            fn to_identifier(&self) -> Identifier {
                self.0.clone()
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
                Some( self.directory.to_identifier() )
            }
        }

        impl MsiDao for DirectoryDao {

            fn conflicts_with(&self, other: &Self) -> bool {
                self.directory == other.directory
            }

            fn to_row(&self) -> Vec<msi::Value> {
                vec![
                    msi::ToValue::to_value(&self.directory),
                    msi::ToValue::to_value(&self.parent_directory),
                    msi::ToValue::to_value(&self.default_dir),
                ]
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct DirectoryTable {
            entries: Vec<DirectoryDao>,
        }

        impl MsiTableKind for DirectoryTable {
            type TableValue = DirectoryDao;
            fn name(&self) -> &'static str {
                "Directory"
            }

            fn entries(&self) -> &Vec<DirectoryDao> {
                &self.entries
            }

            fn entries_mut(&mut self) -> &mut Vec<DirectoryDao> {
                &mut self.entries
            }

            fn len(&self) -> usize {
                self.entries.len()
            }

            fn is_empty(&self) -> bool {
                self.len() == 0
            }

            fn primary_key_indices(&self) -> Vec<usize> {
                vec![0usize,]
            }

            fn columns(&self) -> Vec<msi::Column> {
                vec![
                    msi::Column::build("Directory").primary_key().category(msi::Category::Identifier).string(72),
                    msi::Column::build("Directory_Parent").nullable().foreign_key("Directory", 0).category(msi::Category::Identifier).string(72),
                    msi::Column::build("DefaultDir").localizable().category(msi::Category::DefaultDir).string(255),
                ]
            }
        }

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
                    msi::ToValue::to_value(&self.feature_),
                    msi::ToValue::to_value(&self.component_),
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
