use std::path::Path;
use std::{fs, process};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: bioniconv <file>");
        process::exit(1);
    }
    let fname = &*args[1];

    let ofname = &output_file_name(fname).unwrap_or_else(|| {
        eprintln!("Cannot determine output file name");
        process::exit(1);
    });

    let f = &fs::File::open(fname).unwrap_or_else(|err| {
        eprintln!("Cannot open file: {}", err);
        process::exit(1);
    });
    let of = &fs::File::create(ofname).unwrap_or_else(|err| {
        eprintln!("Cannot open output file {}: {}", ofname, err);
        process::exit(1);
    });

    bioniconv::process_epub(of, f).unwrap_or_else(|err| {
        eprintln!("Convert error: {}", err);
        process::exit(1);
    });
}

fn output_file_name(fname: &str) -> Option<String> {
    Path::new(fname)
        .file_name()
        .map(|name| format!("bionic_{}", name.to_string_lossy()))
}
