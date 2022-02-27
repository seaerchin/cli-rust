use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

fn is_match(regs: &Vec<Regex>, s: &str) -> bool {
    regs.len() == 0 || regs.iter().any(|r| r.is_match(s))
}

pub fn get_args() -> Res<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .about("Rust find")
        .arg(Arg::with_name("paths").multiple(true).default_value("."))
        .arg(
            Arg::with_name("name")
                .takes_value(true)
                .multiple(true)
                .short("n")
                .long("name"),
        )
        .arg(
            Arg::with_name("type")
                .takes_value(true)
                .multiple(true)
                .short("t")
                .long("type")
                .value_name("TYPE")
                .possible_values(&["d", "l", "f"]),
        )
        .get_matches();

    let paths = matches.values_of_lossy("paths").unwrap();
    let names: Result<Vec<_>, String> = matches
        .values_of_lossy("name")
        .unwrap_or(Vec::new())
        .iter()
        .map(|s| Regex::new(s).map_err(|_| format!("Invalid --name \"{}\"", s)))
        .collect();

    // NOTE: This is defaulted to file
    let entry_types = matches
        .values_of_lossy("type")
        .unwrap_or(vec!["d".to_string(), "l".to_string(), "f".to_string()])
        .iter()
        .map(|t| {
            let s: &str = t;
            match s {
                "d" => Dir,
                "l" => Link,
                "f" => File,
                _ => panic!("impossible!"),
            }
        })
        .collect();

    Ok(Config {
        paths,
        names: names?,
        entry_types,
    })
}

pub fn run(config: Config) -> Res<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    let filetype = entry.file_type();
                    let filename = entry.file_name().to_string_lossy();
                    if filetype.is_dir() && config.entry_types.contains(&Dir) {
                        if is_match(&config.names, &filename) {
                            println!("{}", entry.path().display())
                        }
                    }

                    if filetype.is_symlink() && config.entry_types.contains(&Link) {
                        if is_match(&config.names, &filename) {
                            println!("{}", entry.path().display())
                        }
                    }

                    if filetype.is_file() && config.entry_types.contains(&File) {
                        if is_match(&config.names, &filename) {
                            println!("{}", entry.path().display())
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
