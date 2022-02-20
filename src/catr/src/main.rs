use catr;

fn main() {
    let config = catr::get_args();
    if let Ok(c) = config {
        if let Err(e) = catr::run(&c) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
