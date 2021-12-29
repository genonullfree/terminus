use std::path::PathBuf;
use structopt::StructOpt;

mod utils;

/// Terminus is an application for quickly locating exported or imported functions in Elf files

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// List of function names to search for
    #[structopt(short, long, default_value = "")]
    search: Vec<String>,

    /// Directory to search in
    #[structopt(short, long, default_value = "/lib")]
    dir: PathBuf,

    /// Match imported instead of exported functions
    #[structopt(short, long)]
    imported: bool,
}

fn main() {
    // Process arguments
    let opt = Opt::from_args();

    // Scan Elf exports
    utils::scan_elf_exports(opt);
}
