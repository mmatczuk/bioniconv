use lol_html::html_content::ContentType;
use lol_html::{
    text, HandlerTypes, HtmlRewriter, LocalHandlerTypes, OutputSink, RewriteStrSettings,
};
use std::io::Write;
use std::{error, io};
use zip::read::ZipFile;
use zip::ZipWriter;

pub fn process_file<W: Write + io::Seek>(
    w: &mut ZipWriter<W>,
    r: &mut ZipFile,
) -> Result<(), Box<dyn error::Error>> {
    let mut rewriter = HtmlRewriterWrapper {
        inner: HtmlRewriter::new(
            RewriteStrSettings {
                element_content_handlers: vec![text!("p", |t| {
                    t.replace(&format!("XXXX({})", t.as_str()), ContentType::Html);
                    Ok(())
                })],
                ..RewriteStrSettings::new()
            }
            .into(),
            |c: &[u8]| w.write_all(c).unwrap(),
        ),
    };

    io::copy(r, &mut rewriter)?;

    Ok(())
}

// Wrapper around HtmlRewriter that implements io::Write for use with io::copy.
struct HtmlRewriterWrapper<'h, O: OutputSink, H: HandlerTypes = LocalHandlerTypes> {
    inner: HtmlRewriter<'h, O, H>,
}
impl<'h, O: OutputSink, H: HandlerTypes> Write for HtmlRewriterWrapper<'h, O, H> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner.write(buf) {
            Ok(_) => Ok(buf.len()),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
