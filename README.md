# Terminus

Terminus is an application that locates the ELF files that import or export a specific function. This can be used to quickly identify which library you may want to look at next when reversing a new software system by pinpointing the library that exports a specific version of a function that you are looking for.

## Usage

```bash
USAGE:
    terminus [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Prints help information
    -i, --imported    Match imported instead of exported functions
    -V, --version     Prints version information

OPTIONS:
    -d, --dir <dir>             Directory to search in [default: /lib]
    -s, --search <search>...    List of function names to search for [default: ]
```
## Dependencies

Radare2 will need to be installed as this application utilizes r2pipes. Instructions to install Radare2 can be found [here](https://github.com/radareorg/radare2/blob/master/INSTALL.md)

## Installation

If you have Rust and Cargo installed, Terminus can be quickly compiled and installed by running the following command:
```bash
cargo install terminus
```
This will install Terminus to ~/.cargo/bin/terminus, which might need to be added to your shell's PATH variable.

## Example output

```
$ terminus -s printf -d /bin
[!] /bin/lucious matched printf
nth paddr      vaddr      bind   type size lib name
252 0x000049d5 0x000049d5 GLOBAL FUNC   245      cterm_printf_ts
[!] /bin/make matched printf
nth paddr      vaddr      bind   type size lib name
425 0x00011a40 0x00011a40 GLOBAL FUNC   321      file_timestamp_sprintf
[+] Scanned file: /bin/newusers
```

### Getting rid of R2 warnings
```bash
echo 'e bin.cache=true' > ~/.radare2rc
```
