use std::collections::HashMap;
use std::default;
use std::env;
use std::fs::read_to_string;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::Context;
use anyhow::ensure;
use camino::Utf8PathBuf;
use itertools::Itertools;
use regex::Regex;
use ron::Options;
use ron::extensions::Extensions;
use ron::to_string;
use walkdir::WalkDir;
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
use crate::config::ShortcutConfigInfo;
use crate::constants::*;

#[derive(Clone, Debug, Default, PartialEq, strum::Display, clap::ValueEnum)]
pub(crate) enum PathRelativity {
    Config,
    #[default]
    Command,
    // TODO: Can't use non-unit variants with ValueEnum and I can't think of a
    // fast way to enable it.
    // Custom(PathBuf),
}

pub(super) struct Builder {}

impl Builder {
    pub fn build_from_config(
        path_relativity: &PathRelativity,
        config_path: &Utf8PathBuf,
        output_path: &Utf8PathBuf,
    ) -> anyhow::Result<()> {
        // Parse the config
        let options = Options::default()
            .with_default_extension(Extensions::IMPLICIT_SOME)
            .with_default_extension(Extensions::UNWRAP_NEWTYPES);
        let config =
            options.from_str::<MsiConfig>(&read_to_string(config_path)?)?;

        let output_path = if output_path.is_dir() {
            &output_path
                .join(config.summary.subject.clone())
                .with_extension(MSI_EXTENSION)
        } else {
            output_path
        };

        let base_path = match path_relativity {
            PathRelativity::Config => config_path
                .parent()
                .expect("Config path doesn't have a parent")
                .canonicalize_utf8()
                .expect("Failed to get full path of config"),
            PathRelativity::Command => Utf8PathBuf::from_path_buf(
                env::current_dir()
                    .expect("Failed to get current working directory"),
            )
            .expect("Current working directory path is not UTF-8 compatible"),
        };

        // Build the MSI
        let msi_builder = builder_from_config(&config, &base_path)?;

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

fn builder_from_config(
    config: &MsiConfig,
    base_path: &Utf8PathBuf,
) -> anyhow::Result<MsiBuilder> {
    let meta = MetaInformation::new(
        whimsi_msi::PackageType::Installer,
        config.summary.subject.clone(),
    )
    .with_author(Some(config.summary.author.clone()))
    .with_comments(config.summary.comments.clone());
    let mut builder = MsiBuilder::default().with_meta(meta);
    add_properties(&mut builder, &config.properties)?;
    add_paths(&mut builder, base_path, &config.paths, &config.properties)?;
    add_shortcuts(
        &mut builder,
        base_path,
        &config.shortcuts,
        &config.properties,
        &config.paths,
    )?;
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
    base_path: &Utf8PathBuf,
    paths: &HashMap<Utf8PathBuf, String>,
    properties: &HashMap<String, String>,
) -> anyhow::Result<()> {
    for (source_path, destination_path) in paths {
        let source_path = base_path.join(source_path);
        ensure!(
            source_path.exists(),
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
    base_path: &Utf8PathBuf,
    shortcuts: &Vec<ShortcutConfigInfo>,
    properties: &HashMap<String, String>,
    paths: &HashMap<Utf8PathBuf, String>,
) -> anyhow::Result<()> {
    for shortcut in shortcuts {
        let target = &shortcut.target;
        let location = PathBuf::from_str(&shortcut.location).unwrap();
        let destination_directory_id = get_directory_id_of_path(
            &location, builder, properties,
        )
        .unwrap_or_else(|| {
            panic!(
                "Failed to get destination directory ID for path {location:?}"
            )
        });

        // TODO: Fix this ugliness
        if let Some(icon_path) = &shortcut.icon_path {
            let icon_path =
                find_icon_path(icon_path.clone(), base_path, paths)?;
            builder.add_shortcut_with_icon(
                destination_directory_id,
                Shortcut::Formatted(Formatted::from(target.to_string())),
                &Into::<PathBuf>::into(icon_path),
            )?;
        } else {
            builder.add_shortcut(
                destination_directory_id,
                Shortcut::Formatted(Formatted::from(target.to_string())),
            )?;
        }
    }
    Ok(())
}

fn find_icon_path(
    icon_path: Utf8PathBuf,
    base_path: &Utf8PathBuf,
    paths: &HashMap<Utf8PathBuf, String>,
) -> anyhow::Result<Utf8PathBuf> {
    // If there is more than one component to the path then this must be a
    // relative or full path so we should not randomly search for the
    // filename in all paths.
    if icon_path.components().collect_vec().len() > 1 {
        return Ok(base_path.join(icon_path));
    }

    let icon_filename = icon_path
        .file_name()
        .unwrap_or_else(|| {
            panic!("No filename found for icon path {icon_path:?}")
        })
        .to_string();
    let files = paths
        .keys()
        .flat_map(|path| {
            WalkDir::new(base_path.join(path))
                .into_iter()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path().to_path_buf())
        })
        .collect_vec();
    let corrected_path = files
        .into_iter()
        .filter(move |path| {
            *path.file_name().unwrap_or_default().to_string_lossy().to_string()
                == *icon_filename.clone()
        })
        .exactly_one()
        .with_context(|| {
            format!("Error finding {} in paths {:?}", icon_path, paths.keys())
        })?;
    Ok(Utf8PathBuf::from_path_buf(corrected_path.clone()).unwrap_or_else(
        |_| panic!("Path for icon {corrected_path:?} is not valid UTF-8"),
    ))
}

fn get_directory_id_of_path(
    path: &Path,
    builder: &mut MsiBuilder,
    properties: &HashMap<String, String>,
) -> Option<DirectoryIdentifier> {
    let last_component =
        get_last_component(path.to_string_lossy().as_ref(), properties)?;
    if let Ok(system_folder) = SystemFolder::from_str(&last_component) {
        // Ignore any errors creating the system_folder as it's entirely likely
        // it's already been made
        let _ = builder.add_directory_dao(system_folder.into());
        return Some(system_folder.to_identifier().into());
    }

    builder
        .directory()
        .entry_with_name(&DefaultDir::Filename(
            Filename::from_str(&last_component).unwrap(),
        ))
        .map(|d| d.directory().clone())
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
            .unwrap_or_else(|| {
                panic!("Path {path} doesn't have any components")
            })
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
