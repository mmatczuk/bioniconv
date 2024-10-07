use anyhow::Result;
use lol_html::html_content::ContentType;
use lol_html::{
    text, HandlerTypes, HtmlRewriter, LocalHandlerTypes, OutputSink, RewriteStrSettings,
};
use std::io;
use std::io::{Read, Write};

pub fn rewrite_to_bionic<W: Write, R: Read>(w: &mut W, r: &mut R) -> Result<()> {
    let bionic = BionicReplacer::new();
    let mut rewriter = HtmlRewriterWrapper {
        inner: HtmlRewriter::new(
            RewriteStrSettings {
                element_content_handlers: vec![text!("p,li", |t| {
                    let text = t.as_str();
                    if !text.is_empty() {
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
            re: regex::Regex::new(r"(^|\s)(\b[\p{L}\p{M}]+\b)").unwrap(),
        }
    }

    fn replace(&self, text: &str) -> String {
        self.re.replace_all(text, BionicWordReplacer).to_string()
    }
}

struct BionicWordReplacer;

impl regex::Replacer for BionicWordReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        if caps.len() == 0 {
            return;
        }

        let space = caps.get(1).unwrap().as_str();
        dst.push_str(space);

        let word = caps.get(2).unwrap().as_str();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_to_bionic() {
        let tests = vec![
            ("<p>hello world</p>", "<p><b>he</b>llo <b>wo</b>rld</p>"),
            ("<li>hello</li>", "<li><b>he</b>llo</li>"),
        ];

        for (input, expected) in tests {
            let mut input = input.as_bytes();
            let mut output = Vec::new();
            rewrite_to_bionic(&mut output, &mut input).unwrap();
            assert_eq!(String::from_utf8(output).unwrap(), expected);
        }
    }
}
