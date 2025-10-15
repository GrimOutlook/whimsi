use camino::Utf8PathBuf;
use clap::Parser;
use clap::Subcommand;
use clap::arg;
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
        config: Utf8PathBuf,
        /// The relative root to use when generating full paths from the given
        /// config.
        #[arg(short, long)]
        wk_dir: Option<Utf8PathBuf>,
        /// Filepath to output. This should end with .msi
        #[arg(short, long)]
        output: Utf8PathBuf,
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
