use crate::*;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use std::io;
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

pub fn load_dyns(opt: Opt) -> Result<(), Error> {
    let files = get_file_list(&opt);

    if files.is_empty() {
        println!("[!] No files located");
        return Ok(());
    }

    // For each filepath in the input vector...
    for (_num, item) in files.iter().enumerate() {
        if read_elf(item, &opt.search).is_ok() {
            execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
            print!("\r[+] Scanning file: {}", item);
            io::stdout().flush().unwrap();
        }
    }

    Ok(())
}
