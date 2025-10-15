use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::ensure;
use camino::Utf8PathBuf;
use itertools::Itertools;
use regex::Regex;
use ron::to_string;
use whimsi_lib::builder::MsiBuilder;
use whimsi_lib::tables::directory::directory_identifier::DirectoryIdentifier;
use whimsi_lib::tables::meta::MetaInformation;
use whimsi_lib::tables::service_control::event::Event;
use whimsi_lib::types::column::default_dir::DefaultDir;
use whimsi_lib::types::column::filename::Filename;
use whimsi_lib::types::column::formatted::Formatted;
use whimsi_lib::types::column::identifier::Identifier;
use whimsi_lib::types::column::identifier::ToIdentifier;
use whimsi_lib::types::column::shortcut::Shortcut;
use whimsi_lib::types::helpers::to_unique_msi_identifier::ToUniqueMsiIdentifier;
use whimsi_lib::types::properties::system_folder::SystemFolder;

use crate::config::MsiConfig;
use crate::config::Permission;
use crate::config::ServiceInstallConfigInfo;

pub(super) struct Builder {}

impl Builder {
    pub fn build_from_config(
        config_path: &Utf8PathBuf,
        output_path: &Utf8PathBuf,
    ) -> anyhow::Result<()> {
        // Parse the config
        let config = ron::from_str::<MsiConfig>(&read_to_string(config_path)?)?;

        // Build the MSI
        let msi_builder = builder_from_config(&config)?;

        // Open the output file
        let file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;

        // Write the MSI to the output file
        msi_builder.build(file)?;
        Ok(())
    }
}

fn builder_from_config(config: &MsiConfig) -> anyhow::Result<MsiBuilder> {
    let meta = MetaInformation::new(
        whimsi_msi::PackageType::Installer,
        config.summary.subject.clone(),
    )
    .with_author(Some(config.summary.author.clone()))
    .with_comments(config.summary.comments.clone());
    let mut builder = MsiBuilder::default().with_meta(meta);
    add_properties(&mut builder, &config.properties)?;
    add_paths(&mut builder, &config.paths, &config.properties)?;
    add_shortcuts(&mut builder, &config.shortcuts, &config.properties)?;
    add_services(&mut builder, &config.service_installs, &config.properties)?;
    add_permissions(&mut builder, &config.permissions, &config.properties)?;

    Ok(builder)
}
fn add_properties(
    builder: &mut MsiBuilder,
    properties: &HashMap<String, String>,
) -> anyhow::Result<()> {
    for (key, value) in properties {
        builder.add_property(key, value)?
    }
    Ok(())
}

fn add_paths(
    builder: &mut MsiBuilder,
    paths: &HashMap<Utf8PathBuf, String>,
    properties: &HashMap<String, String>,
) -> anyhow::Result<()> {
    for (source_path, destination_path) in paths {
        println!("Adding path: {}", source_path);
        ensure!(
            source_path.try_exists().unwrap(),
            "Source path {} doesn't exist",
            source_path
        );

        let destination_path_id = add_components_to_directory_table(
            builder,
            properties,
            SystemFolder::TARGETDIR.to_string(),
            destination_path,
        )?;
        builder.add_path_contents(source_path, destination_path_id)?;
    }
    Ok(())
}

// NOTE: My brain has shut off for some reason so this likely needs to be
// redone. Works during testing though.
fn add_components_to_directory_table(
    builder: &mut MsiBuilder,
    properties: &HashMap<String, String>,
    initial_parent: String,
    path: &String,
) -> anyhow::Result<DirectoryIdentifier> {
    let path = PathBuf::from(path);
    let mut parent_id = DirectoryIdentifier::from_str(&initial_parent)?;
    let parsed_components = path
        .components()
        .rev()
        .map(|component| {
            let component = component.as_os_str().to_string_lossy();
            get_component_value(component.to_string(), properties)
        })
        .flatten_ok()
        .collect::<anyhow::Result<Vec<String>>>()?
        .into_iter();

    for component in parsed_components {
        parent_id =
            if let Ok(system_folder) = SystemFolder::from_str(&component) {
                builder.add_directory_dao(system_folder.into()).unwrap();
                DirectoryIdentifier::from_str(&system_folder.to_string())?
            } else {
                builder.add_directory(&component, parent_id.clone()).unwrap()
            };
    }
    Ok(parent_id)
}

fn get_reserved_property(prop: &str) -> Option<String> {
    static RESERVED_PROPERTY: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\[\[([A-Za-z0-9_]+)\]\]").unwrap());
    if let Some(captures) = RESERVED_PROPERTY.captures(prop)
        && let Some(property) = captures.get(1)
    {
        let Ok(system_folder) = SystemFolder::from_str(property.as_str())
        else {
            panic!(
                "Property is not a valid reserved property: {}",
                property.as_str()
            );
        };
        Some(system_folder.to_string())
    } else {
        None
    }
}

fn get_component_value(
    component: String,
    properties: &HashMap<String, String>,
) -> anyhow::Result<Vec<String>> {
    static CUSTOM_PROPERTY: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\[([A-Za-z0-9_]+)\]").unwrap());

    let value = if let Some(system_folder) = get_reserved_property(&component) {
        vec![system_folder]
    } else if let Some(captures) = CUSTOM_PROPERTY.captures(&component)
        && let Some(property) = captures.get(1)
    {
        PathBuf::from_str(properties.get(property.as_str()).unwrap_or_else(
            || panic!("Failed to get property: {}", property.as_str()),
        ))?
        .components()
        .map(|c| {
            get_component_value(
                c.as_os_str().to_string_lossy().to_string(),
                properties,
            )
        })
        .flatten_ok()
        .collect::<anyhow::Result<Vec<String>>>()?
    } else {
        vec![component]
    };
    Ok(value)
}

fn add_shortcuts(
    builder: &mut MsiBuilder,
    shortcuts: &HashMap<String, String>,
    properties: &HashMap<String, String>,
) -> anyhow::Result<()> {
    for (source_path, destination_path) in shortcuts {
        let destination_path = PathBuf::from_str(destination_path).unwrap();
        let destination_directory_id = get_directory_id_of_path(
            destination_path.parent().unwrap(),
            builder,
            properties,
        )
        .unwrap();
        builder.add_shortcut(
            destination_directory_id,
            Filename::from_str(
                destination_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .as_ref(),
            )?,
            Shortcut::Formatted(Formatted::from(source_path.to_string())),
        )?;
    }
    Ok(())
}

fn get_directory_id_of_path(
    path: &Path,
    builder: &mut MsiBuilder,
    properties: &HashMap<String, String>,
) -> Option<DirectoryIdentifier> {
    let last_component =
        get_last_component(path.to_string_lossy().as_ref(), properties)?;
    let directory_with_name = builder
        .directory()
        .entry_with_name(&DefaultDir::Filename(
            Filename::from_str(&last_component).unwrap(),
        ))
        .map(|d| d.directory().clone());
    match directory_with_name {
        Some(d) => Some(d),
        None => {
            if let Ok(system_folder) = SystemFolder::from_str(&last_component) {
                builder.add_directory_dao(system_folder.into()).unwrap();
                Some(system_folder.to_identifier().into())
            } else {
                None
            }
        }
    }
}

fn add_services(
    builder: &mut MsiBuilder,
    service_installs: &[ServiceInstallConfigInfo],
    properties: &HashMap<String, String>,
) -> anyhow::Result<()> {
    for service in service_installs {
        let service_id = builder.add_service_install(
            service.name.clone().into(),
            service.typ,
            service.start_time,
            service.error_control,
        )?;

        if let Some(control) = &service.control {
            let mut event = Event::empty();
            // TODO: Man this is ugly 😞. The things I do to make things work
            // quickly 🤮.
            match control.start {
                crate::config::ControlCondition::Never => (),
                crate::config::ControlCondition::Install => {
                    event |= Event::INSTALL_START
                }
                crate::config::ControlCondition::Uninstall => {
                    event |= Event::UNINSTALL_START
                }
                crate::config::ControlCondition::InstallAndUninstall => {
                    event |= Event::INSTALL_START & Event::UNINSTALL_START
                }
            }
            match control.stop {
                crate::config::ControlCondition::Never => (),
                crate::config::ControlCondition::Install => {
                    event |= Event::INSTALL_STOP
                }
                crate::config::ControlCondition::Uninstall => {
                    event |= Event::UNINSTALL_STOP
                }
                crate::config::ControlCondition::InstallAndUninstall => {
                    event |= Event::INSTALL_STOP & Event::UNINSTALL_STOP
                }
            }
            match control.remove {
                crate::config::ControlCondition::Never => (),
                crate::config::ControlCondition::Install => {
                    event |= Event::INSTALL_DELETE
                }
                crate::config::ControlCondition::Uninstall => {
                    event |= Event::UNINSTALL_DELETE
                }
                crate::config::ControlCondition::InstallAndUninstall => {
                    event |= Event::INSTALL_DELETE & Event::UNINSTALL_DELETE
                }
            }

            builder.add_service_control(
                service.name.clone().into(),
                event,
                control.wait,
            )?;
        }
    }
    Ok(())
}
fn add_permissions(
    builder: &mut MsiBuilder,
    permissions: &HashMap<String, Permission>,
    properties: &HashMap<String, String>,
) -> anyhow::Result<()> {
    for (path, permission) in permissions {
        // Determine if this is a valid file or directory.
        let last_component = get_last_component(path, properties)
            .unwrap_or_else(|| todo!("Create a real error"));
        let filename = Filename::from_str(&last_component)
            .unwrap_or_else(|_| todo!("Create a real error"));
        let (lock_object, table) = if let Some(file) =
            builder.file().entry_with_name(&filename)
        {
            (file.file().clone().into(), "File")
        } else if let Some(directory) =
            builder.directory().entry_with_name(&filename.into())
        {
            // TODO: Add support for directory permissions. These can only be
            // applied to CreateFolder table listings which is currently not
            // supported.
            todo!(
                "Need to add support for CreateFolder table before this feature can be enabled."
            );
        } else {
            todo!(
                "Create an error for when the path to set permissions on is not found"
            )
        };

        // Add the permission to the builder.
        builder.add_lock_permissions(
            lock_object,
            permission.user.clone().into(),
            permission.level.clone(),
        )?;
    }

    Ok(())
}

fn get_last_component(
    path: &str,
    properties: &HashMap<String, String>,
) -> Option<String> {
    let last_component = get_component_value(
        PathBuf::from_str(path)
            .ok()?
            .components()
            .next_back()
            .unwrap()
            .as_os_str()
            .to_string_lossy()
            .to_string(),
        properties,
    )
    .ok()?
    .last()
    .unwrap()
    .to_string();
    Some(last_component)
}
