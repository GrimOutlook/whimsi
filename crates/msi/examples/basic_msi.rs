use std::{fs::File, path::PathBuf};

use clap::Parser;
use clap_derive::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    msi: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file = File::create(args.msi).expect("Failed to create file");
    let package =
        whimsi_msi::Package::create(whimsi_msi::PackageType::Installer, file)
            .unwrap();

    // [This
    // documentation](https://learn.microsoft.com/en-us/windows/win32/msi/summary-property-descriptions)
    // says that the REQUIRED SummaryInformation for an MSI of the Installer type is:
    // - Template (Made up of Language and Architecture)
    // - Revision Number
    // - Page Count
    // - Word Count
}
