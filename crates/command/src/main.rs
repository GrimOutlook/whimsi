#![deny(unsafe_code)]
#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_variables, unreachable_code, unused_imports)
)]
#![cfg_attr(not(debug_assertions), deny(warnings, unused_crate_dependencies))]
mod builder;
mod command_line;
pub mod enums;
/// TODO: Deal with config after getting initial functionality working. Getting bogged down
/// bikeshedding.
// mod config;
mod inspector;
mod scan;
mod models {
    pub(crate) mod directory;
    pub(crate) mod file;
    pub mod sequencer;
}
mod traits {
    pub(crate) mod identifier;
}

use anyhow::Result;
use builder::MsiBuilder;
use clap::Parser;
use command_line::{CommandLineParser, Commands, Listable};
use tracing::info;
use tracing_subscriber::{FmtSubscriber, util::SubscriberInitExt};

fn main() -> Result<()> {
    // Read the passed in arguments
    let args = CommandLineParser::parse();
    // Setup the logger
    FmtSubscriber::builder().with_max_level(args.log_level).finish().init();

    info!("Running WHIMSI...");
    match args.command {
        Commands::Build {
            config_path: config,
            input_directory,
            output_path,
        } => {
            // let builder = MsiBuilder::new()?;
            todo!()
        }
        Commands::Inspect { input_file, list_args } => {
            let output = inspector::inspect(&input_file, list_args)?;
            println!("{output}");
        }
    };

    info!("whimsi operation succeeded");
    Ok(())
}
