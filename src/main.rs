use elf_rs::*;
use r2pipe::R2Pipe;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

mod utils;

/// Terminus is an application for quickly locating exported functions in Elf libraries

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// List of function names to search for
    #[structopt(short, long, default_value = "")]
    search: Vec<String>,

    /// Directory to search in
    #[structopt(short, long, default_value = "/lib")]
    dir: PathBuf,
}

fn read_elf(filename: &str, search: &[String]) -> Result<(), Error> {
    // TODO: Find better way to validate Elf files
    let mut elf_file = File::open(filename).unwrap();
    let mut elf_buf = Vec::<u8>::new();
    elf_file.read_to_end(&mut elf_buf).unwrap();
    let _ = Elf::from_bytes(&elf_buf)?;

    // Scan Elf file for exported symbols
    let mut r2p = R2Pipe::spawn(filename, None).unwrap();
    r2p.cmd("af").unwrap();
    let out = r2p.cmd("iE").unwrap();

    for i in search {
        if out.contains(i) {
            println!("\r[!] {} exports {}", filename, i);
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    // Process arguments
    let opt = Opt::from_args();
    utils::load_dyns(opt)?;

    Ok(())
}
