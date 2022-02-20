use std::{
    error::Error,
    fs,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

pub type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

// 1: Get the (possibly empty) text contained in each file
// 2: Check if should number_lines || number_non_blank
// 3: Pretty print
pub fn run(config: &Config) -> Res<()> {
    let contents: Vec<_> = config.files.iter().map(|filepath| open(filepath)).collect();

    for res in contents {
        match res {
            Ok(contents) => {
                print_output(contents, config.number_lines, config.number_nonblank_lines)
            }
            Err(e) => eprintln!("catr: {e} - No such file or directory"),
        }
    }
    Ok(())
}

// If we read successfully, the caller should own the result.
// Otherwise, we can return the path as is.
fn open(path: &str) -> Result<Box<dyn BufRead>, &str> {
    if path == "" || path == "-" {
        return Ok(Box::new(BufReader::new(io::stdin())));
    }

    let file_result = fs::File::open(path);
    match file_result {
        Ok(file) => Ok(Box::new(BufReader::new(file))),
        Err(_) => Err(path),
    }
}

fn print_output<T: BufRead>(reader: T, number_lines: bool, number_nonblank: bool) {
    let mut idx = 0;
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            // check if we need to number
            if number_nonblank && !line.is_empty() {
                println!("{:>6}\t{}", idx, line);
                idx += 1;
            } else if number_lines {
                println!("{:>6}\t{}", idx, line);
                idx += 1;
            } else {
                println!("{}", line);
            }
        }
    }
}

pub fn get_args() -> Res<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("chin")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .required(true)
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .min_values(1),
        )
        .arg(Arg::with_name("number_lines").short("n").long("number"))
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
