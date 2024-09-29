use std::{error, io};

pub fn process_epub<R: io::Read + io::Seek>(reader: R) -> Result<(), Box<dyn error::Error>> {
    let a = zip::ZipArchive::new(reader).unwrap();
    println!("{}", a.len());
    Ok(())
}
