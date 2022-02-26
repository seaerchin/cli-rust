use clap::{App, Arg};
use std::{error::Error, fs::File, io::BufRead};

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
    repeats: bool,
    uniques: bool,
    case: bool,
}

impl Config {
    fn get_counter(&self) -> Counter {
        todo!()
    }
}

pub struct Counter {
    count: bool,
    repeats: bool,
    uniques: bool,
    case: bool,
}

impl Counter {
    fn new(c: &Config) -> Self {
        Counter {
            count: c.count,
            repeats: c.repeats,
            uniques: c.uniques,
            case: c.case,
        }
    }

    fn count(self, source: &str) -> Vec<(u128, &str)> {
        todo!()
    }
}

pub fn get_args() -> Res<Config> {
    let matches = App::new("uniqr")
        .about("Rust uniq")
        .arg(Arg::with_name("count").short("c").takes_value(false))
        .arg(Arg::with_name("input").default_value("-").takes_value(true))
        .arg(Arg::with_name("output").requires("input").takes_value(true))
        .arg(Arg::with_name("repeats").short("d").takes_value(false))
        .arg(Arg::with_name("uniques").short("u").takes_value(false))
        .arg(Arg::with_name("case").short("i").takes_value(false))
        .get_matches();

    todo!()
}

pub fn run(config: Config) -> Res<()> {
    println!("{:?}", config);
    todo!()
}

fn write(path: &str, data: &str) {
    let f = File::create(path);

    todo!()
}

// This can fail
fn open(path: &str) -> Res<Box<dyn BufRead>> {
    todo!()
}
