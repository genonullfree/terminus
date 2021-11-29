use std::io::Read;
use std::fs::File;
use std::env;

use elf_rs::*;

fn read_elf(filename: &String) {
    let mut elf_file = File::open(filename).unwrap();
    let mut elf_buf = Vec::<u8>::new();
    elf_file.read_to_end(&mut elf_buf).unwrap();

    let elf = Elf::from_bytes(&elf_buf).unwrap();

    if let Elf::Elf64(e) = elf {
        println!("{:?} header: {:?}", e, e.header());

        for p in e.program_header_iter() {
            println!("{:x?}", p);
        }

        for s in e.section_header_iter() {
            println!("{:x?}", s);
        }

        let s = e.lookup_section(".text");
        println!("{:?}", s);
    }
}

fn main() {
    read_elf(&"file.elf".to_string());
}
