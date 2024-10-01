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
    let bionic = BionicReplacer::new();
    let mut rewriter = HtmlRewriterWrapper {
        inner: HtmlRewriter::new(
            RewriteStrSettings {
                element_content_handlers: vec![text!("p", |t| {
                    let text = t.as_str();
                    if text.len() > 0 {
                        t.replace(&bionic.replace(text), ContentType::Html);
                    }
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

struct BionicReplacer {
    re: regex::Regex,
}

impl BionicReplacer {
    fn new() -> Self {
        Self {
            re: regex::Regex::new(r"(\b[\p{L}\p{M}]+\b)").unwrap(),
        }
    }

    fn replace(&self, text: &str) -> String {
        self.re.replace_all(text, BionicWordReplacer).to_string()
    }
}

struct BionicWordReplacer;

impl regex::Replacer for BionicWordReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let word = caps.get(1).unwrap().as_str();
        if word.len() <= 1 {
            dst.push_str(word);
        } else if word.len() <= 3 {
            dst.push_str("<b>");
            dst.push_str(&word[..1]);
            dst.push_str("</b>");
            dst.push_str(&word[1..]);
        } else {
            let midpoint = word.len() / 2;
            dst.push_str("<b>");
            dst.push_str(&word[..midpoint]);
            dst.push_str("</b>");
            dst.push_str(&word[midpoint..]);
        }
    }
}
