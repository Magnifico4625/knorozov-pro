//! PDF export (A4) with embedded Inter font (OFL-1.1) — full Cyrillic support.
use anyhow::{anyhow, Result};
use krilla::color::rgb;
use krilla::geom::Point;
use krilla::metadata::Metadata;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::Fill;
use krilla::surface::Surface;
use krilla::text::{Font, TextDirection};
use krilla::Document;

use super::DocMeta;
use crate::timefmt;
use crate::transcript::Transcript;

pub static FONT_REGULAR: &[u8] = include_bytes!("../../assets/fonts/Inter-Regular.ttf");
pub static FONT_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/Inter-SemiBold.ttf");

const PAGE_W: f32 = 595.28;
const PAGE_H: f32 = 841.89;
const MARGIN_X: f32 = 60.0;
const MARGIN_TOP: f32 = 64.0;
const MARGIN_BOTTOM: f32 = 64.0;

/// Text measurement using the font's horizontal advances.
struct Metrics {
    face: ttf_parser::Face<'static>,
}

impl Metrics {
    fn new(data: &'static [u8]) -> Result<Self> {
        Ok(Self { face: ttf_parser::Face::parse(data, 0).map_err(|e| anyhow!("font: {e}"))? })
    }
    fn width(&self, s: &str, size: f32) -> f32 {
        let upm = self.face.units_per_em() as f32;
        s.chars()
            .map(|c| {
                self.face
                    .glyph_index(c)
                    .and_then(|g| self.face.glyph_hor_advance(g))
                    .unwrap_or((upm * 0.5) as u16) as f32
            })
            .sum::<f32>()
            * size
            / upm
    }
}

/// Greedy word wrap.
fn wrap(m: &Metrics, text: &str, size: f32, max_w: f32) -> Vec<String> {
    let mut lines = Vec::new();
    for raw_line in text.lines() {
        let mut cur = String::new();
        for word in raw_line.split_whitespace() {
            let candidate = if cur.is_empty() { word.to_string() } else { format!("{cur} {word}") };
            if m.width(&candidate, size) <= max_w || cur.is_empty() {
                // very long single word: hard-break by characters
                if cur.is_empty() && m.width(word, size) > max_w {
                    let mut piece = String::new();
                    for ch in word.chars() {
                        piece.push(ch);
                        if m.width(&piece, size) > max_w {
                            piece.pop();
                            lines.push(std::mem::take(&mut piece));
                            piece.push(ch);
                        }
                    }
                    cur = piece;
                } else {
                    cur = candidate;
                }
            } else {
                lines.push(std::mem::replace(&mut cur, word.to_string()));
            }
        }
        lines.push(cur);
    }
    lines
}

fn fill(r: u8, g: u8, b: u8) -> Fill {
    Fill { paint: rgb::Color::new(r, g, b).into(), opacity: NormalizedF32::ONE, rule: Default::default() }
}

struct Writer {
    doc: Document,
    regular: Font,
    bold: Font,
    pages: Vec<Vec<Item>>,
    y: f32,
}

enum Item {
    Text { x: f32, y: f32, size: f32, bold: bool, color: (u8, u8, u8), text: String },
}

impl Writer {
    fn ensure(&mut self, h: f32) {
        if self.pages.is_empty() || self.y + h > PAGE_H - MARGIN_BOTTOM {
            self.pages.push(Vec::new());
            self.y = MARGIN_TOP;
        }
    }
    fn text(&mut self, x: f32, size: f32, bold: bool, color: (u8, u8, u8), text: &str) {
        let y = self.y;
        self.pages.last_mut().unwrap().push(Item::Text { x, y, size, bold, color, text: text.to_string() });
    }
    fn render(mut self) -> Result<Vec<u8>> {
        let total = self.pages.len();
        let pages = std::mem::take(&mut self.pages);
        for (pi, items) in pages.into_iter().enumerate() {
            let mut page = self
                .doc
                .start_page_with(PageSettings::from_wh(PAGE_W, PAGE_H).ok_or_else(|| anyhow!("page"))?);
            let mut surface: Surface = page.surface();
            for it in items {
                let Item::Text { x, y, size, bold, color, text } = it;
                surface.set_fill(Some(fill(color.0, color.1, color.2)));
                let font = if bold { self.bold.clone() } else { self.regular.clone() };
                surface.draw_text(Point::from_xy(x, y), font, size, &text, false, TextDirection::Auto);
            }
            if total > 1 {
                surface.set_fill(Some(fill(156, 163, 175)));
                let label = format!("{} / {}", pi + 1, total);
                surface.draw_text(
                    Point::from_xy(PAGE_W / 2.0 - 12.0, PAGE_H - 32.0),
                    self.regular.clone(),
                    8.0,
                    &label,
                    false,
                    TextDirection::Auto,
                );
            }
            surface.finish();
            page.finish();
        }
        self.doc.finish().map_err(|e| anyhow!("pdf: {e:?}"))
    }
}

pub fn to_pdf(t: &Transcript, meta: &DocMeta) -> Result<Vec<u8>> {
    let regular = Font::new(FONT_REGULAR.to_vec().into(), 0).ok_or_else(|| anyhow!("font"))?;
    let bold = Font::new(FONT_SEMIBOLD.to_vec().into(), 0).ok_or_else(|| anyhow!("font"))?;
    let m_reg = Metrics::new(FONT_REGULAR)?;
    let mut doc = Document::new();
    let mut md = Metadata::new().producer("Кнорозов PRO".to_string()).creator("Кнорозов PRO".to_string());
    if !meta.title.is_empty() {
        md = md.title(meta.title.clone());
    }
    doc.set_metadata(md);
    let mut w = Writer { doc, regular, bold, pages: Vec::new(), y: MARGIN_TOP };
    let max_w = PAGE_W - 2.0 * MARGIN_X;

    let text_size = 11.0;
    let line_h = text_size * 1.5;

    if !meta.title.is_empty() {
        let m_bold = Metrics::new(FONT_SEMIBOLD)?;
        for line in wrap(&m_bold, &meta.title, 18.0, max_w) {
            w.ensure(26.0);
            w.y += 18.0;
            w.text(MARGIN_X, 18.0, true, (17, 24, 39), &line);
            w.y += 8.0;
        }
    }
    if !meta.subtitle.is_empty() {
        w.ensure(18.0);
        w.y += 10.0;
        w.text(MARGIN_X, 10.0, false, (107, 114, 128), &meta.subtitle);
        w.y += 10.0;
    }
    w.y += 10.0;

    for p in &t.paragraphs {
        let lines = wrap(&m_reg, p.text.trim(), text_size, max_w);
        // keep header with at least two lines of text
        w.ensure(28.0 + line_h * lines.len().min(2) as f32);
        w.y += 20.0;
        let tc = timefmt::chip(p.start_ms);
        w.text(MARGIN_X, 9.0, false, (37, 99, 235), &tc);
        if let Some(name) = t.speaker_name(p.speaker) {
            w.text(MARGIN_X + m_reg.width(&tc, 9.0) + 12.0, 10.5, true, (17, 24, 39), name);
        }
        w.y += 4.0;
        for line in lines {
            w.ensure(line_h);
            w.y += line_h;
            w.text(MARGIN_X, text_size, false, (31, 41, 55), &line);
        }
        w.y += 6.0;
    }
    if w.pages.is_empty() {
        w.ensure(0.0);
    }
    w.render()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf_has_extractable_cyrillic() {
        let t = crate::export::testdata::sample();
        let meta = DocMeta { title: "Встреча команды".into(), subtitle: "06.10.2026 · 28:14".into() };
        let bytes = to_pdf(&t, &meta).unwrap();
        assert!(bytes.starts_with(b"%PDF-"));
        let text = pdf_extract::extract_text_from_mem(&bytes).unwrap();
        let norm: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(norm.contains("Встреча команды"), "{norm}");
        assert!(norm.contains("00:12:45"), "{norm}");
        assert!(norm.contains("Иван"), "{norm}");
        assert!(norm.contains("Мы сегодня обсуждаем"), "{norm}");
        assert!(norm.contains("Ёлка"), "{norm}");
    }

    #[test]
    fn pdf_paginates_long_text() {
        let mut t = crate::export::testdata::sample();
        let long = "Длинный абзац текста для проверки переноса строк и разбиения на страницы. ".repeat(400);
        t.set_paragraph_text(0, &long);
        let bytes = to_pdf(&t, &DocMeta::default()).unwrap();
        let text = pdf_extract::extract_text_from_mem(&bytes).unwrap();
        assert!(text.contains("1 / "), "page numbers expected");
    }

    #[test]
    fn wrap_respects_width() {
        let m = Metrics::new(FONT_REGULAR).unwrap();
        let lines = wrap(&m, &"слово ".repeat(100), 11.0, 300.0);
        assert!(lines.len() > 5);
        for l in &lines {
            assert!(m.width(l, 11.0) <= 300.0 + 0.01);
        }
    }
}
