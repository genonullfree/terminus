use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

use elf_rs::*;

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

fn read_elf(filename: &str) -> Result<(), Error> {
    let mut elf_file = File::open(filename).unwrap();
    let mut elf_buf = Vec::<u8>::new();
    elf_file.read_to_end(&mut elf_buf).unwrap();

    let elf = Elf::from_bytes(&elf_buf)?;

    if let Elf::Elf64(e) = elf {
        //        println!("{:?} header: {:?}", e, e.header());

        for p in e.program_header_iter() {
            //            println!("{:x?}", p);
        }

        for s in e.section_header_iter() {
            //            println!("{:x?}", s);
        }

        let s = e.lookup_section(".text");
        //        println!("{:?}", s);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    // Process arguments
    let opt = Opt::from_args();
    utils::load_dyns(opt)?;

    Ok(())
}
