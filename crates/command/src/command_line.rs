use camino::Utf8PathBuf;
use clap::{arg, Parser, Subcommand};
use flexstr::SharedStr;
use tracing::Level;

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
        #[arg(short, long)]
        config_path: Utf8PathBuf,
        /// Directory storing files used to be added to MSI
        #[arg(short, long)]
        input_directory: Utf8PathBuf,
        /// Filepath to output. This should end with .msi
        #[arg(short, long)]
        output_path: Utf8PathBuf,
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
