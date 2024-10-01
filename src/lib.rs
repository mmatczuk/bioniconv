use lol_html::html_content::ContentType;
use lol_html::{text, HtmlRewriter, RewriteStrSettings};
use std::io::{Read, Write};
use std::{error, io};
use zip::read::ZipFile;
use zip::write::SimpleFileOptions;
use zip::{self, DateTime, ZipArchive, ZipWriter};

pub fn process_epub<W, R>(w: W, r: R) -> Result<(), Box<dyn error::Error>>
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
            let mut f = za.by_name(&fname)?;
            zw.start_file(f.name(), copy_options(&f))?;
            process_file(&mut zw, &mut f)?;
        } else {
            let f = za.by_name(&fname)?;
            zw.raw_copy_file(f)?;
        }
    }

    zw.finish()?;
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

fn process_file<W: Write + io::Seek>(
    w: &mut ZipWriter<W>,
    r: &mut ZipFile,
) -> Result<(), Box<dyn error::Error>> {
    let mut content = String::new();
    r.read_to_string(&mut content)?;

    let mut rewriter = HtmlRewriter::new(
        RewriteStrSettings {
            element_content_handlers: vec![text!("p", |t| {
                t.replace(&format!("XXXX({})", t.as_str()), ContentType::Html);
                Ok(())
            })],
            ..RewriteStrSettings::new()
        }
        .into(),
        |c: &[u8]| w.write_all(c).unwrap(),
    );

    rewriter.write(content.as_bytes())?;
    rewriter.end()?;

    Ok(())
}
