use camino::Utf8PathBuf;
use clap::Parser;
use clap::Subcommand;
use clap::arg;
use flexstr::SharedStr;
use tracing::Level;

use crate::builder::PathRelativity;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct CommandLineParser {
    #[arg(long, default_value_t = Level::WARN)]
    pub(crate) log_level: Level,
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Build {
        /// Path to config to build from
        config: Utf8PathBuf,
        /// Filepath to output.
        /// If this points to an existing directory, the created file will be
        /// named [PROGRAM_NAME].msi in the directory. Otherwise the
        /// file is created at the given path as given.
        output: Utf8PathBuf,
        /// The relative root to use when generating full paths from the given
        /// config.
        #[arg(short, long, default_value = "command")]
        relative_to: PathRelativity,
    },
    Inspect {
        /// MSI to inspect
        input_file: Utf8PathBuf,

        #[command(subcommand)]
        list_args: Listable,
    },
}

#[derive(Subcommand)]
#[group(required = true, multiple = false)]
pub(crate) enum Listable {
    // List the author of the MSI
    Author,
    // List tables present in the MSI
    Tables,
    // Tables with at least one entry
    NonEmptyTables,
    // List the columns that a given table has.
    TableColumns { table: SharedStr },
    // List the contents of a given table
    TableContents { table: SharedStr },
}
