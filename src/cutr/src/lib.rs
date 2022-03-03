use crate::Extract::*;
use clap::{App, Arg};
use std::{error::Error, ops::Range};

type Res<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

pub fn get_args() -> Res<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .about("Rust cut")
        .arg(Arg::with_name("files").multiple(true).default_value("-"))
        .arg(
            Arg::with_name("delim")
                .short("d")
                .long("delim")
                .takes_value(true)
                .default_value("\t"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("b")
                .long("bytes")
                .multiple(true)
                .use_delimiter(true)
                .conflicts_with_all(&["chars, fields"]),
        )
        .arg(
            Arg::with_name("chars")
                .short("c")
                .long("chars")
                .multiple(true)
                .use_delimiter(true)
                .conflicts_with_all(&["bytes, fields"]),
        )
        .arg(
            Arg::with_name("fields")
                .short("f")
                .long("fields")
                .multiple(true)
                .use_delimiter(true)
                .conflicts_with_all(&["bytes, chars"]),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let delim = matches.value_of("delim").unwrap();
    let extract: Extract = [
        matches.values_of_lossy("fields"),
        matches.values_of_lossy("bytes"),
        matches.values_of_lossy("chars"),
    ]
    .iter()
    .reduce(todo!())
    .unwrap();

    Ok(Config {
        files,
        // Always guaranteed to exist as delim is defaulted to \t
        delimiter: delim.as_bytes()[0],
        extract,
    })
}

pub fn to_extract_type(s: &str, list: PositionList) -> Extract {
    match s {
        "f" => Fields(list),
        "c" => Chars(list),
        "b" => Bytes(list),
        _ => todo!(),
    }
}

pub fn run(c: Config) -> Res<()> {
    println!("{:#?}", c);
    Ok(())
}

fn to_position_list(ls: Vec<String>) -> PositionList {
    todo!()
}
