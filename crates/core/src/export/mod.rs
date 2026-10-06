//! Exporters: DOCX, PDF, TXT, SRT.

pub mod docx;
pub mod pdf;
pub mod srt;
pub mod txt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Docx,
    Pdf,
    Txt,
    Srt,
}

impl ExportFormat {
    pub fn extension(self) -> &'static str {
        match self {
            ExportFormat::Docx => "docx",
            ExportFormat::Pdf => "pdf",
            ExportFormat::Txt => "txt",
            ExportFormat::Srt => "srt",
        }
    }
}

/// Document header info shown in DOCX/PDF.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocMeta {
    pub title: String,
    /// e.g. «06.10.2026 · 1:24:17 · русский»
    pub subtitle: String,
}

pub fn export_bytes(
    t: &crate::transcript::Transcript,
    meta: &DocMeta,
    fmt: ExportFormat,
) -> anyhow::Result<Vec<u8>> {
    Ok(match fmt {
        ExportFormat::Docx => docx::to_docx(t, meta)?,
        ExportFormat::Pdf => pdf::to_pdf(t, meta)?,
        ExportFormat::Txt => txt::to_txt(t).into_bytes(),
        ExportFormat::Srt => srt::to_srt(t).into_bytes(),
    })
}

#[cfg(test)]
pub(crate) mod testdata {
    use crate::transcript::{Segment, Transcript};

    pub fn sample() -> Transcript {
        let seg = |s: u64, e: u64, t: &str, sp: usize| Segment {
            start_ms: s,
            end_ms: e,
            text: t.into(),
            speaker: Some(sp),
        };
        let mut t = Transcript::from_segments(
            "ru",
            30_000,
            vec![
                seg(765_000, 768_500, "Мы сегодня обсуждаем важные вопросы, связанные с развитием проекта.", 0),
                seg(768_500, 772_000, "Во-первых, стоит отметить значительный прогресс.", 0),
                seg(808_000, 812_000, "Да, согласен. Особенно хочу выделить работу команды «Ёлка» №1.", 1),
                seg(850_000, 853_000, "Hello world — English too.", 0),
            ],
        );
        t.rename_speaker(0, "Иван");
        t
    }
}
