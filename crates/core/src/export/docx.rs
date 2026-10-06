//! Word (.docx) export with timecodes and speaker labels.
use anyhow::Result;
use docx_rs::{Docx, Paragraph as DPara, Run, RunFonts};

use super::DocMeta;
use crate::timefmt;
use crate::transcript::Transcript;

fn font() -> RunFonts {
    RunFonts::new().ascii("Calibri").hi_ansi("Calibri").cs("Calibri").east_asia("Calibri")
}

pub fn to_docx(t: &Transcript, meta: &DocMeta) -> Result<Vec<u8>> {
    let mut doc = Docx::new();
    if !meta.title.is_empty() {
        doc = doc.add_paragraph(
            DPara::new().add_run(Run::new().add_text(&meta.title).bold().size(32).fonts(font())),
        );
    }
    if !meta.subtitle.is_empty() {
        doc = doc.add_paragraph(
            DPara::new().add_run(Run::new().add_text(&meta.subtitle).size(20).color("6B7280").fonts(font())),
        );
    }
    for p in &t.paragraphs {
        let mut head = DPara::new().add_run(
            Run::new().add_text(timefmt::chip(p.start_ms)).size(18).color("2563EB").fonts(font()),
        );
        if let Some(name) = t.speaker_name(p.speaker) {
            head = head
                .add_run(Run::new().add_text("   ").size(18))
                .add_run(Run::new().add_text(name).bold().size(22).fonts(font()));
        }
        doc = doc.add_paragraph(head);
        doc = doc.add_paragraph(DPara::new().add_run(Run::new().add_text(p.text.trim()).size(22).fonts(font())));
    }
    let mut buf = std::io::Cursor::new(Vec::new());
    doc.build().pack(&mut buf)?;
    Ok(buf.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn docx_contains_cyrillic_timecodes_speakers() {
        let t = crate::export::testdata::sample();
        let meta = DocMeta { title: "Интервью с экспертом".into(), subtitle: "06.10.2026 · 28:14 · русский".into() };
        let bytes = to_docx(&t, &meta).unwrap();
        assert_eq!(&bytes[..2], b"PK");
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let mut xml = String::new();
        zip.by_name("word/document.xml").unwrap().read_to_string(&mut xml).unwrap();
        assert!(xml.contains("Интервью с экспертом"));
        assert!(xml.contains("00:12:45"));
        assert!(xml.contains("Иван"));
        assert!(xml.contains("Спикер 2"));
        assert!(xml.contains("«Ёлка» №1"));
        assert!(xml.contains("Мы сегодня обсуждаем важные вопросы"));
    }
}
