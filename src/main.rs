mod builder;
mod command_line;
mod config;
mod lister;
mod scan;
pub(crate) mod tables;
mod models {
    pub(crate) mod directory;
    pub(crate) mod file;
    pub mod sequencer;
}
mod traits {
    pub(crate) mod identifier;
}

use std::process::ExitCode;

use clap::Parser;
use command_line::{CommandLineParser, Commands, Listable};
use tracing::{error, info};
use tracing_subscriber::{FmtSubscriber, util::SubscriberInitExt};

fn main() -> ExitCode {
    // Read the passed in arguments
    let args = CommandLineParser::parse();
    // Setup the logger
    FmtSubscriber::builder().with_max_level(args.log_level).finish().init();

    info!("Running msipmbuild...");
    let ret = match args.command {
        Commands::Build { config, input_directory, output_path } => {
            builder::build(&config, &input_directory, &output_path)
        }
        Commands::Inspect { input_file, list_args } => {
            match lister::list(&input_file, list_args) {
                Ok(output) => {
                    println!("{output}");
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    };

    match ret {
        Ok(_) => (),
        Err(e) => {
            error!("msipmbuild operation failed. Error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    info!("msipmbuild operation succeeded");
    ExitCode::SUCCESS
}
