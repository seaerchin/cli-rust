use clap::{App, Arg};
use std::{
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Stdin},
};

pub type Res<T> = Result<T, Box<dyn Error>>;

enum InputType {
    File((BufReader<File>, String)),
    Stdin(BufReader<Stdin>),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

impl Read for InputType {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        match self {
            InputType::File((f, _)) => f.read(buf),
            InputType::Stdin(s) => s.read(buf),
        }
    }
}

impl BufRead for InputType {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            InputType::File((f, _)) => f.fill_buf(),
            InputType::Stdin(s) => s.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            InputType::File((f, _)) => f.consume(amt),
            InputType::Stdin(s) => s.consume(amt),
        }
    }
}

impl InputType {
    fn print(&self) {
        if let InputType::File((_, filename)) = self {
            // print an empty line before this and terminate the output with a newline
            println!("\n==> {} <==", filename);
        }
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
                .default_value("-")
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
        files: matches
            .values_of_lossy("files")
            .or_else(|| Some(vec!["-".to_string()]))
            .unwrap(),
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
    let num_files = c.files.len();
    for filename in c.files {
        match open(&filename) {
            Ok(file) => {
                if num_files > 1 {
                    file.print();
                }

                print_output(file, c.lines, c.bytes)
            }
            Err(e) => eprintln!("{}: {}", filename, e),
        }
    }
    Ok(())
}

fn open(path: &str) -> Res<Box<InputType>> {
    if path == "" || path == "-" {
        return Ok(Box::new(InputType::Stdin(BufReader::new(std::io::stdin()))));
    }

    let file_result = fs::File::open(path);
    match file_result {
        Ok(file) => {
            return Ok(Box::new(InputType::File((
                BufReader::new(file),
                String::from(path),
            ))))
        }
        Err(e) => return Err(Box::new(e)),
    };
}

fn print_output<T: BufRead>(reader: T, num_lines: usize, num_bytes: Option<usize>) {
    // this is pretty bad code tbh
    if let Some(b) = num_bytes {
        // take the required number of bytes
        let bytes: Vec<_> = reader
            .bytes()
            .filter(|i| i.is_ok())
            .take(b)
            .map(|i| i.unwrap())
            .collect();

        let s = String::from_utf8_lossy(&bytes);

        for c in s.chars().into_iter() {
            print!("+{}", (c));
        }
    } else {
        for line_result in reader.lines().take(num_lines) {
            match line_result {
                Ok(line) => println!("{}", line),
                Err(_) => println!("Something"),
            }
        }
    }
}
