use std::{error, io};
use zip::{ZipArchive, ZipWriter};

pub fn process_epub<W, R>(w: W, r: R) -> Result<(), Box<dyn error::Error>>
where
    W: io::Write + io::Seek,
    R: io::Read + io::Seek,
{
    let za = ZipArchive::new(r)?;
    let zw = ZipWriter::new(w);

    for fname in za.file_names() {
        if fname.ends_with("content.opf") {
            continue;
        } else if [".html", ".xhtml", "htm"]
            .iter()
            .any(|s| fname.ends_with(s))
        {
            println!("process {}", fname);
        } else {
            println!("copy {}", fname);
        }
    }

    Ok(())
}
