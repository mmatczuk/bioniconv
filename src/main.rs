use bionicread;
use std::{fs, process};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: bionicread <file>");
        process::exit(1);
    }

    let fname = &*args[1];
    let f = fs::File::open(fname).unwrap_or_else(|err| {
        eprintln!("Cannot open file: {}", err);
        process::exit(1);
    });

    bionicread::process_epub(f).unwrap_or_else(|err| {
        eprintln!("Convert error: {}", err);
        process::exit(1);
    });
}
