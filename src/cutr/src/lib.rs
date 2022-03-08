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
    let extract: Extract;

    if let Some(v) = matches.values_of_lossy("fields") {
        extract = Fields(to_position_list(v));
    } else if let Some(v) = matches.values_of_lossy("bytes") {
        extract = Bytes(to_position_list(v));
    } else {
        let v = matches.values_of_lossy("chars").unwrap();
        extract = Chars(to_position_list(v));
    }

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
    // There are 4 possible cases here for raw cli input
    // 1: x - y -> [x, y]
    // 2: x, y, z
    // 3: x
    // 4: a-b, c-d, e-f

    // This is case 1 | 3
    if ls.len() == 1 {
        return vec![to_range(&ls[0])];
    } else {
        return ls.iter().map(|s| to_range(s)).collect();
    }
}

// converts a string of either x | x - y into a suitable range
fn to_range(s: &str) -> Range<usize> {
    let split: Vec<_> = s.split("-").collect();
    // exactly 1 item
    if split.len() == 1 {
        let lower: usize = split[0].parse().unwrap();
        return lower..lower + 1;
    }

    // 2 items separated by -
    let lower: usize = split[0].parse().unwrap();
    let upper: usize = split[1].parse().unwrap();
    return lower..upper + 1;
}

pub fn parse_pos(range: &str) -> Res<PositionList> {
    unimplemented!();
}
