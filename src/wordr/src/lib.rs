use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
    ops::Add,
};

type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Option<Vec<String>>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub struct WcReader {
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

// lines, words, byte || char
#[derive(Clone)]
struct WcResult(Vec<u128>);

impl WcResult {
    fn format(&self) -> String {
        self.0.iter().map(|num| format!("{:>8}", num)).collect()
    }

    fn new(v: Vec<u128>) -> Self {
        WcResult(v)
    }
}

impl Add for WcResult {
    type Output = WcResult;

    fn add(self, other: WcResult) -> WcResult {
        let v = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(first, second)| first + second)
            .collect();

        WcResult::new(v)
    }
}

impl WcReader {
    // takes a filepath and returns the formatted wc
    fn wc(&self, source: &str) -> Res<WcResult> {
        let mut reader = open(source)?;
        let mut buf = vec![];
        let mut str_buf = vec![];

        let _ = reader.read_to_end(&mut str_buf);
        let string = String::from_utf8_lossy(&str_buf);

        if self.lines {
            buf.push(count_lines(&string));
        }
        if self.words {
            buf.push(count_words(&string));
        }

        // Because the two are conflicting, only 1 of self.bytes || self.chars will be true
        if self.bytes {
            buf.push(count_bytes(&string));
        } else if self.chars {
            buf.push(count_chars(&string));
        }

        Ok(WcResult::new(buf))
    }

    fn new(c: &Config) -> Self {
        return WcReader {
            lines: c.lines,
            words: c.words,
            bytes: c.bytes,
            chars: c.chars,
        };
    }
}

impl Config {
    fn get_reader(&self) -> WcReader {
        return WcReader::new(&self);
    }
}

pub fn get_args() -> Res<Config> {
    let matches = App::new("wcr")
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .takes_value(false)
                .conflicts_with("chars"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("files")
                .multiple(true)
                .value_name("FILES")
                .help("the files to read from"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut bytes = matches.is_present("bytes");
    let mut words = matches.is_present("words");

    let chars = matches.is_present("chars");
    let files = matches.values_of_lossy("files");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        lines,
        bytes,
        chars,
        words,
        files,
    })
}

pub fn run(config: Config) -> Res<()> {
    let reader = config.get_reader();

    match config.files {
        Some(files) => {
            let data: Vec<(Res<WcResult>, &String)> =
                files.iter().map(|path| (reader.wc(path), path)).collect();

            let mut count = WcResult::new(vec![0, 0, 0]);

            for res in data {
                let (wc_res, filepath) = res;
                match wc_res {
                    Ok(res) => {
                        println!("{} {}", res.format(), filepath);
                        count = count + res;
                    }
                    Err(e) => eprintln!("wcr: {}: {}", filepath, e),
                }
            }

            if files.len() > 1 {
                println!("{} total", count.format());
            }
        }

        None => {
            println!("{}", reader.wc("").unwrap().format());
        }
    };

    Ok(())
}

trait Default<T> {
    fn default(self) -> bool;
}

impl Default<&str> for Option<&str> {
    fn default(self) -> bool {
        return self.map(|_| true).unwrap_or(false);
    }
}

fn count_bytes(source: &str) -> u128 {
    source.as_bytes().len().try_into().unwrap()
}

fn count_lines(source: &str) -> u128 {
    let v: Vec<_> = source.lines().collect();
    v.iter().len().try_into().unwrap()
}

fn count_chars(source: &str) -> u128 {
    source
        .chars()
        .collect::<Vec<_>>()
        .iter()
        .len()
        .try_into()
        .unwrap()
}

fn count_words(source: &str) -> u128 {
    source
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .len()
        .try_into()
        .unwrap()
}

fn open(path: &str) -> Res<Box<dyn BufRead>> {
    if path == "" {
        return Ok(Box::new(BufReader::new(stdin())));
    }

    match File::open(path) {
        Ok(f) => Ok(Box::new(BufReader::new(f))),
        Err(e) => Err(Box::new(e)),
    }
}
