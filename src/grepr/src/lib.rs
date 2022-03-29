use clap::{App, Arg};
use predicates::boolean;
use regex::{Error as RegexError, Regex, RegexBuilder};
use std::error::Error;

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> Res<Config> {
    // Step 1: build the parser and run it against the command line
    let matches = App::new("grepr")
        .version("0.1.0")
        .arg(Arg::with_name("count").short("c").long("count"))
        .arg(Arg::with_name("insensitive").short("i").long("insensitive"))
        .arg(Arg::with_name("invert").short("v").long("invert-match"))
        .arg(Arg::with_name("recursive").short("r").long("recursive"))
        .arg(Arg::with_name("pattern").required(true))
        .arg(
            Arg::with_name("files")
                .default_value("-")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    // guaranteed safe as this is required by the parser
    let base = matches.value_of("pattern").unwrap();
    let is_insensitive = matches.is_present("insensitive");
    let pattern = extract_regex(base, is_insensitive);
    let files = matches.values_of_lossy("files").unwrap_or(vec![]);
    let recursive = matches.is_present("recursive");
    let count = matches.is_present("count");
    let invert = matches.is_present("invert");

    Ok(Config {
        pattern,
        files,
        recursive,
        count,
        invert_match: invert,
    })
}

pub fn run(config: Config) -> Res<()> {
    todo!()
}

fn extract_regex(base: &str, is_insensitive: bool) -> Res<Regex> {
    let actual_base = if is_insensitive {
        base.to_lowercase()
    } else {
        String::from(base)
    };

    Regex::new(&actual_base).map_err(|e| Box::new(e))
}
