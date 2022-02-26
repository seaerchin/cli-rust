use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    vec,
};

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
        Counter::new(&self)
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

    fn count<'a>(&self, source: &'a str) -> Vec<(u128, &'a str)> {
        source.lines().fold(Vec::new(), |mut acc, cur| {
            if acc.len() == 0 {
                return vec![(1, cur)];
            }

            // guaranteed since length != 0
            let (count, last_elem) = acc.pop().unwrap();
            let is_equal = if self.case {
                last_elem.to_ascii_lowercase() == cur.to_ascii_lowercase()
            } else {
                last_elem == cur
            };

            if is_equal {
                acc.push((count + 1, cur));
                return acc;
            }

            acc.push((count, last_elem));
            acc.push((1, cur));
            acc
        })
    }

    fn filter<'a>(&self, data: Vec<(u128, &'a str)>) -> Vec<(u128, &'a str)> {
        if self.repeats {
            data.into_iter().filter(|(count, _)| count > &1).collect()
        } else if self.uniques {
            data.into_iter()
                .filter(|(count, _)| *count == 1 as u128)
                .collect()
        } else {
            data
        }
    }

    fn format(&self, data: Vec<(u128, &str)>) -> String {
        self.filter(data)
            .into_iter()
            .map(|(count, s)| {
                format!(
                    "{}{}",
                    if self.count {
                        format!("{:>4} ", count)
                    } else {
                        "".to_string()
                    },
                    s
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn get_args() -> Res<Config> {
    let matches = App::new("uniqr")
        .about("Rust uniq")
        .arg(Arg::with_name("count").short("c").takes_value(false))
        .arg(Arg::with_name("input").default_value("-").takes_value(true))
        .arg(Arg::with_name("output").requires("input").takes_value(true))
        .arg(
            Arg::with_name("repeats")
                .short("d")
                .takes_value(false)
                .conflicts_with("uniques"),
        )
        .arg(Arg::with_name("uniques").short("u").takes_value(false))
        .arg(Arg::with_name("case").short("i").takes_value(false))
        .get_matches();

    let count = matches.is_present("count");
    let in_file = matches.value_of("input").unwrap().to_string();
    let out_file = matches.value_of("output").map(|x| x.to_string());
    let repeats = matches.is_present("repeats");
    let uniques = matches.is_present("uniques");
    let case = matches.is_present("case");

    Ok(Config {
        in_file,
        out_file,
        count,
        repeats,
        uniques,
        case,
    })
}

pub fn run(config: Config) -> Res<()> {
    let config = get_args()?;
    let counter = config.get_counter();
    let mut buf: Vec<u8> = vec![];
    let _ = open(&config.in_file)?.read_to_end(&mut buf);

    let s = String::from_utf8_lossy(&buf);
    let counts = counter.count(&s);
    let display_data = counter.format(counts);

    write(&config.out_file.unwrap_or("".to_string()), &display_data)
}

fn write(path: &str, data: &str) -> Res<()> {
    let mut f: Box<dyn Write> = if path == "" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(path)?)
    };

    f.write_all(str::as_bytes(data));
    Ok(())
}

// This can fail
fn open(path: &str) -> Res<Box<dyn BufRead>> {
    if path == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        match File::open(path) {
            Ok(f) => Ok(Box::new(BufReader::new(f))),
            Err(e) => Err(Box::new(e)),
        }
    }
}
