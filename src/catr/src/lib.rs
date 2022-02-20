use std::{
    error::Error,
    fs,
    io::{self, BufReader},
};

use clap::{App, Arg};

type Res<T> = Result<T, Box<dyn Error>>;

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
    let contents: Vec<Result<String, &str>> =
        config.files.iter().map(|filepath| open(filepath)).collect();

    for res in contents {
        match res {
            Ok(contents) => {
                let output =
                    get_output(contents, config.number_lines, config.number_nonblank_lines);
                print!("{}", output)
            }
            Err(e) => eprintln!("catr: {e} - No such file or directory"),
        }
    }
    Ok(())
}

// If we read successfully, the caller should own the result.
// Otherwise, we can return the path as is.
fn open(path: &str) -> Result<String, &str> {
    if path == "" || path == "-" {
        let reader = Ok::<_, i32>(BufReader::new(io::stdin()));
        todo!()
    }
    fs::read_to_string(path).map_err(|_| path)
}

// 1: Split by new lines
// 2: Check if we only need the nonempty lines
// 3: Zip with input then map to the output string
fn get_output(contents: String, number_lines: bool, number_nonblank: bool) -> String {
    contents
        .split("\n")
        .filter(|line| {
            if number_nonblank {
                !line.is_empty()
            } else {
                true
            }
        })
        .zip(0..)
        .map(|(line, line_no)| {
            format!(
                "{}{}\n",
                if number_lines || number_nonblank {
                    format!("{} ", line_no.to_string())
                } else {
                    String::from("")
                },
                line
            )
        })
        .collect()
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
