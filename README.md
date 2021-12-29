# Terminus

Terminus is an application that locates the library files that export a specific function. This can be used to quickly identify which library you may want to look at next when reversing a new software system by pinpointing the library that exports a specific version of a function that you are looking for.

## Usage

```bash
USAGE:
    terminus [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <dir>             Directory to search in [default: /lib]
    -s, --search <search>...    List of function names to search for [default: ]
```

### Getting rid of R2 warndings
```bash
echo 'e bin.cache=true' > ~/.radare2rc
```
