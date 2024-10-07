use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("usage: bioniconv <file>");
    }
    let fname = &*args[1];

    let ofname = &output_file_name(fname)
        .with_context(|| format!("determine output file name for {}", fname))?;

    let f = &fs::File::open(fname).with_context(|| format!("open input file {}", fname))?;

    let of = &fs::File::create(ofname).with_context(|| format!("open output file {}", ofname))?;

    bioniconv::process_epub(of, f)?;

    Ok(())
}

fn output_file_name(fname: &str) -> Option<String> {
    Path::new(fname)
        .file_name()
        .map(|name| format!("bionic_{}", name.to_string_lossy()))
}
