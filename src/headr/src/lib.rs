use clap::{App, Arg};
use std::{
    error::Error,
    fs::{self},
    io::{BufRead, BufReader},
    ops::Index,
};

pub type Res<T> = Result<T, Box<dyn Error>>;

type Files = Vec<String>;

#[derive(Debug)]
pub struct Config {
    files: Option<Files>,
    lines: usize,
    bytes: Option<usize>,
}

enum Mode {
    Lines(usize),
    Bytes(usize),
}

fn get_mode(c: &Config) -> Mode {
    if let Some(b) = c.bytes {
        return Mode::Bytes(b);
    }
    return Mode::Lines(c.lines);
}

struct Printer {
    mode: Mode,
}

impl Printer {
    fn new(mode: Mode) -> Printer {
        return Printer { mode };
    }

    fn print(&self, path: Option<String>) {
        let filename = path.clone().unwrap_or("stdin".into());
        match open(&path) {
            Ok(reader) => match self.mode {
                Mode::Lines(num_lines) => print_lines(reader, num_lines),
                Mode::Bytes(num_bytes) => print_bytes(reader, num_bytes),
            },
            // Couldn't open the file
            Err(e) => eprintln!("{}: {}", filename, e),
        }
    }
}

fn print_lines<R: BufRead>(reader: R, num_lines: usize) {
    for line_result in reader.lines().take(num_lines) {
        match line_result {
            Ok(line) => println!("{}", line),
            Err(_) => println!("Something"),
        }
    }
}

fn print_bytes<R: BufRead>(reader: R, num_bytes: usize) {
    // take the required number of bytes
    let bytes: Vec<_> = reader
        .bytes()
        .filter(|i| i.is_ok())
        .take(num_bytes)
        .map(|i| i.unwrap())
        .collect();

    let s = String::from_utf8_lossy(&bytes);

    for c in s.chars().into_iter() {
        print!("{}", (c));
    }
}

pub fn get_args() -> Res<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("chin")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("input files")
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .help("number of lines to display")
                .value_name("LINES")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("number of bytes to display"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    // technically we can check but if they mess up here, it's easier to panic and restart
    Ok(Config {
        files: matches.values_of_lossy("files"),
        // just panic if they don't pass in a proper value - not much we can do anyway
        lines: lines.unwrap(),
        bytes,
    })
}

fn parse_positive(n: &str) -> Res<usize> {
    match n.parse() {
        Ok(i) => Ok(i),
        _ => Err(n.into()),
    }
}

/**
 * For each file,
 * read out the first n lines/bytes
 */
pub fn run(c: Config) -> Res<()> {
    let mode = get_mode(&c);
    let printer = Printer::new(mode);

    match c.files {
        Some(files) => {
            if files.len() > 1 {
                for file in files {
                    println!("==> {} <==", file);
                    printer.print(Some(file));
                }
            } else {
                printer.print(Some(files.index(0).to_string()));
            }
        }
        None => printer.print(None),
    }

    Ok(())
}

fn open(p: &Option<String>) -> Res<Box<dyn BufRead>> {
    if let Some(path) = p {
        match fs::File::open(path) {
            Ok(f) => return Ok(Box::new(BufReader::new(f))),
            Err(e) => return Err(Box::new(e)),
        }
    }
    return Ok(Box::new(BufReader::new(std::io::stdin())));
}
