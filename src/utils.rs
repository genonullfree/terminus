use crate::*;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use elf_rs::*;
use r2pipe::R2Pipe;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::{stdout, Write};
use std::path::Path;

fn get_file_list(opt: &Opt) -> Vec<String> {
    let mut files = Vec::<String>::new();
    let item = &opt.dir;
    if item.is_dir() {
        let mut tmp = match scope_dir(&item.to_path_buf()) {
            Ok(t) => t,
            Err(_) => {
                println!("Error: Cannot read item: {:?}", item);
                return files;
            }
        };
        files.append(&mut tmp);
    } else if item.exists() && item.is_file() {
        files.push(item.to_str().unwrap().to_string());
    }

    files
}

fn scope_dir(dir: &Path) -> Result<Vec<String>, Error> {
    let path = Path::new(&dir);
    let mut files = Vec::<String>::new();

    for entry in path.read_dir().unwrap() {
        if entry.as_ref().unwrap().file_type().unwrap().is_dir() {
            if entry.as_ref().unwrap().path() == *dir {
                continue;
            }

            let mut tmp = match scope_dir(&entry.as_ref().unwrap().path()) {
                Ok(t) => t,
                Err(_) => {
                    println!("Error: Cannot read dir: {:?}", entry);
                    continue;
                }
            };
            files.append(&mut tmp);
        } else if entry.as_ref().unwrap().file_type().unwrap().is_file() {
            files.push(entry.unwrap().path().to_str().unwrap().to_string());
        }
    }

    Ok(files)
}

pub fn scan_elf_exports(opt: Opt) {
    // Get all files recursively
    let files = get_file_list(&opt);

    // If no files found, we're done
    if files.is_empty() {
        println!("[!] No files located");
        return;
    }

    // For each filepath in the input vector...
    for (_num, item) in files.iter().enumerate() {
        // Scan the file for exported symbols
        if scan_elf(item, &opt.search).is_ok() {
            // Clear the terminal line and print the previously scanned file
            execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
            print!("\r[+] Scanned file: {}", item);
            io::stdout().flush().unwrap();
        }
    }
}

fn scan_elf(filename: &str, search: &[String]) -> Result<(), Error> {
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
            // TODO: Print better info
            println!("\r[!] {} exports {}", filename, i);
        }
    }

    Ok(())
}
