mod html;

use anyhow::{Context, Result};
use std::io;
use zip::read::ZipFile;
use zip::write::SimpleFileOptions;
use zip::{self, DateTime, ZipArchive, ZipWriter};

pub fn process_epub<W, R>(w: W, r: R) -> Result<()>
where
    W: io::Write + io::Seek,
    R: io::Read + io::Seek,
{
    let za = &mut ZipArchive::new(r)?;
    let mut zw = ZipWriter::new(w);

    let files: Vec<String> = za.file_names().map(|s| s.to_string()).collect();
    for fname in files {
        if [".html", ".xhtml", "htm"]
            .iter()
            .any(|s| fname.ends_with(s))
        {
            process_file(&mut zw, za, &fname)
                .with_context(|| format!("processing file {}", fname))?;
        } else {
            copy_file(&mut zw, za, &fname).with_context(|| format!("copying file {}", fname))?;
        }
    }

    zw.finish()?;
    Ok(())
}

fn process_file<W, R>(zw: &mut ZipWriter<W>, za: &mut ZipArchive<R>, fname: &str) -> Result<()>
where
    W: io::Write + io::Seek,
    R: io::Read + io::Seek,
{
    let mut f = za.by_name(fname)?;
    zw.start_file(f.name(), copy_options(&f))?;
    html::process_file(zw, &mut f)?;

    Ok(())
}

fn copy_file<W, R>(zw: &mut ZipWriter<W>, za: &mut ZipArchive<R>, fname: &str) -> Result<()>
where
    W: io::Write + io::Seek,
    R: io::Read + io::Seek,
{
    let f = za.by_name(fname)?;
    zw.raw_copy_file(f)?;

    Ok(())
}

fn copy_options(file: &ZipFile) -> SimpleFileOptions {
    let mut options = SimpleFileOptions::default()
        .large_file(file.compressed_size().max(file.size()) > zip::ZIP64_BYTES_THR)
        .last_modified_time(
            file.last_modified()
                .unwrap_or_else(DateTime::default_for_write),
        )
        .compression_method(file.compression());
    if let Some(perms) = file.unix_mode() {
        options = options.unix_permissions(perms);
    }
    options
}
