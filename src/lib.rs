mod html;

use anyhow::{Context, Result};
use std::io;
use zip::read::ZipFile;
use zip::write::SimpleFileOptions;
use zip::{self, DateTime, ZipArchive, ZipWriter};

pub struct EpubRewriter<W, R>
where
    W: io::Write + io::Seek,
    R: io::Read + io::Seek,
{
    zw: ZipWriter<W>,
    za: ZipArchive<R>,
}

impl<W, R> EpubRewriter<W, R>
where
    W: io::Write + io::Seek,
    R: io::Read + io::Seek,
{
    pub fn new(w: W, r: R) -> Result<Self> {
        Ok(Self {
            zw: ZipWriter::new(w),
            za: ZipArchive::new(r)?,
        })
    }

    pub fn rewrite(&mut self) -> Result<()> {
        let files: Vec<String> = self.za.file_names().map(|s| s.to_string()).collect();
        for fname in files {
            if [".html", ".xhtml", "htm"]
                .iter()
                .any(|s| fname.ends_with(s))
            {
                self.process_file(&fname)
                    .with_context(|| format!("processing file {}", fname))?;
            } else {
                self.copy_file(&fname)
                    .with_context(|| format!("copying file {}", fname))?;
            }
        }

        Ok(())
    }

    fn process_file(&mut self, fname: &str) -> Result<()> {
        let mut f = self.za.by_name(fname)?;
        self.zw.start_file(f.name(), copy_options(&f))?;
        html::process_file(&mut self.zw, &mut f)?;

        Ok(())
    }

    fn copy_file(&mut self, fname: &str) -> Result<()> {
        let f = self.za.by_name(fname)?;
        self.zw.raw_copy_file(f)?;

        Ok(())
    }

    pub fn finish(self) -> Result<()> {
        self.zw.finish()?;
        Ok(())
    }
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
