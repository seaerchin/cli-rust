use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Chin <erjiachin@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let should_omit = matches.is_present("omit_newline");
    let input_text = matches.values_of_lossy("text").unwrap();

    let ending = if should_omit { "" } else { "\n" };

    for text in input_text {
        print!("{}{}", text, ending);
    }
}
