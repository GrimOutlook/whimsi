pub(crate) mod files;
pub(crate) mod meta;
pub(crate) mod product;
pub(crate) mod summary;
pub(crate) mod system_folder;

use std::env;

use anyhow::Context;
use camino::Utf8PathBuf;
use files::FilesConfig;
use flexstr::LocalStr;
use meta::MetaConfig;
use product::ProductConfig;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use summary::SummaryConfig;

#[derive(Deserialize)]
pub(crate) struct MsiConfig {
    pub(crate) meta: MetaConfig,
    pub(crate) product_info: ProductConfig,
    pub(crate) summary_info: SummaryConfig,
    pub(crate) files: FilesConfig,
}

#[cfg(test)]
mod tests {
    use assertables::assert_some;
    use similar_asserts::assert_eq;

    use super::*;

    const TEST_CONFIG: &str = r#"
[product_info]
product_name = "Test Application"
product_version = "22.1.15"
manufacturer = "Myself"
product_language = 1033

[summary_info]
page_count = 200
revision_number = "*"
template = "x64;1033"
author = "Test Name"
"#;

    #[test]
    fn config_deserializes() {
        let c: MsiConfig = toml::from_str(TEST_CONFIG).unwrap();

        // Product Info Properties
        assert_eq!(c.product_info.name, "Test Application");
        assert_eq!(c.product_info.version, "22.1.15");
        assert_eq!(c.product_info.manufacturer, "Myself");
        assert_eq!(c.product_info.language, 1033);
        assert_eq!(c.product_info.product_code, None);

        // Summary Info Properties
        assert_eq!(c.summary_info.page_count, 200);
        assert_eq!(c.summary_info.revision_number, "*");
        assert_eq!(c.summary_info.template, "x64;1033");
        assert_some!(c.summary_info.author.clone());
        assert_eq!(c.summary_info.author.unwrap(), "Test Name");
    }
}
