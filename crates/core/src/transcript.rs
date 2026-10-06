//! Transcript data model: Whisper segments grouped into editable paragraphs with speakers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Segment {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
    #[serde(default)]
    pub speaker: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Paragraph {
    pub id: u32,
    pub start_ms: u64,
    pub end_ms: u64,
    /// Index into [`Transcript::speakers`].
    pub speaker: Option<usize>,
    pub text: String,
    /// Original timed segments (used for SRT when the paragraph was not edited).
    pub segments: Vec<Segment>,
    /// True once the user edited `text` by hand.
    #[serde(default)]
    pub edited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Transcript {
    /// Detected or chosen language code (e.g. "ru").
    pub language: String,
    pub duration_ms: u64,
    /// Display names of speakers, e.g. ["Спикер 1", "Иван"]. Empty when diarization is off.
    pub speakers: Vec<String>,
    pub paragraphs: Vec<Paragraph>,
}

/// Paragraph break thresholds.
const PAUSE_BREAK_MS: u64 = 2_000;
const MAX_PARAGRAPH_MS: u64 = 90_000;
const MAX_PARAGRAPH_CHARS: usize = 900;

pub fn default_speaker_name(i: usize) -> String {
    format!("Спикер {}", i + 1)
}

impl Transcript {
    /// Build paragraphs from raw segments. If any segment carries a speaker, paragraphs are
    /// split on speaker change and speaker names are initialised to «Спикер N».
    pub fn from_segments(language: &str, duration_ms: u64, segments: Vec<Segment>) -> Self {
        let segments: Vec<Segment> = segments
            .into_iter()
            .map(|mut s| {
                s.text = clean_text(&s.text);
                s
            })
            .filter(|s| !s.text.is_empty())
            .collect();
        let n_speakers = segments.iter().filter_map(|s| s.speaker).max().map(|m| m + 1).unwrap_or(0);
        let speakers = (0..n_speakers).map(default_speaker_name).collect();

        let mut paragraphs: Vec<Paragraph> = Vec::new();
        for seg in segments {
            let start_new = match paragraphs.last() {
                None => true,
                Some(p) => {
                    let chars: usize = p.text.chars().count();
                    p.speaker != seg.speaker
                        || seg.start_ms.saturating_sub(p.end_ms) > PAUSE_BREAK_MS
                        || seg.end_ms.saturating_sub(p.start_ms) > MAX_PARAGRAPH_MS
                        || chars > MAX_PARAGRAPH_CHARS
                }
            };
            if start_new {
                let id = paragraphs.len() as u32;
                paragraphs.push(Paragraph {
                    id,
                    start_ms: seg.start_ms,
                    end_ms: seg.end_ms,
                    speaker: seg.speaker,
                    text: seg.text.clone(),
                    segments: vec![seg],
                    edited: false,
                });
            } else {
                let p = paragraphs.last_mut().unwrap();
                p.end_ms = p.end_ms.max(seg.end_ms);
                p.text = join_text(&p.text, &seg.text);
                p.segments.push(seg);
            }
        }
        Transcript { language: language.to_string(), duration_ms, speakers, paragraphs }
    }

    /// Rename a speaker; since paragraphs reference speakers by index, the new name is
    /// applied everywhere in the document at once.
    pub fn rename_speaker(&mut self, index: usize, new_name: &str) -> bool {
        let name = new_name.trim();
        if name.is_empty() {
            return false;
        }
        match self.speakers.get_mut(index) {
            Some(s) => {
                *s = name.to_string();
                true
            }
            None => false,
        }
    }

    /// Replace a paragraph's text (user edit in the editor).
    pub fn set_paragraph_text(&mut self, id: u32, text: &str) -> bool {
        if let Some(p) = self.paragraphs.iter_mut().find(|p| p.id == id) {
            let t = text.trim();
            if p.text != t {
                p.text = t.to_string();
                p.edited = true;
            }
            true
        } else {
            false
        }
    }

    pub fn speaker_name(&self, idx: Option<usize>) -> Option<&str> {
        idx.and_then(|i| self.speakers.get(i)).map(|s| s.as_str())
    }

    pub fn has_speakers(&self) -> bool {
        !self.speakers.is_empty()
    }

    /// Full plain text, used for «Копировать всё» and TXT export.
    pub fn plain_text(&self, with_speakers: bool, with_timecodes: bool) -> String {
        let mut out = String::new();
        for (i, p) in self.paragraphs.iter().enumerate() {
            if p.text.trim().is_empty() {
                continue;
            }
            if i > 0 && !out.is_empty() {
                out.push_str("\n\n");
            }
            let mut head = Vec::new();
            if with_timecodes {
                head.push(format!("[{}]", crate::timefmt::chip(p.start_ms)));
            }
            if with_speakers {
                if let Some(name) = self.speaker_name(p.speaker) {
                    head.push(format!("{name}:"));
                }
            }
            if !head.is_empty() {
                out.push_str(&head.join(" "));
                out.push(' ');
            }
            out.push_str(p.text.trim());
        }
        out.push('\n');
        out
    }
}

/// Whisper output cleanup: trim, collapse whitespace, drop pure non-speech markers.
pub fn clean_text(s: &str) -> String {
    let t = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let lower = t.to_lowercase();
    let markers = ["[blank_audio]", "[музыка]", "[music]", "(music)", "[silence]", "[ silence ]", "[тишина]"];
    if markers.iter().any(|m| lower == *m) {
        return String::new();
    }
    t
}

fn join_text(a: &str, b: &str) -> String {
    if a.is_empty() {
        b.to_string()
    } else if b.is_empty() {
        a.to_string()
    } else {
        format!("{a} {b}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(s: u64, e: u64, t: &str, sp: Option<usize>) -> Segment {
        Segment { start_ms: s, end_ms: e, text: t.into(), speaker: sp }
    }

    #[test]
    fn groups_by_speaker_and_pause() {
        let t = Transcript::from_segments(
            "ru",
            20_000,
            vec![
                seg(0, 2000, " Привет.", Some(0)),
                seg(2000, 4000, "Как дела?", Some(0)),
                seg(4100, 6000, "Хорошо.", Some(1)),
                seg(9000, 10000, "Отлично.", Some(1)),
            ],
        );
        assert_eq!(t.speakers, vec!["Спикер 1", "Спикер 2"]);
        assert_eq!(t.paragraphs.len(), 3);
        assert_eq!(t.paragraphs[0].text, "Привет. Как дела?");
        assert_eq!(t.paragraphs[1].speaker, Some(1));
        assert_eq!(t.paragraphs[2].start_ms, 9000);
    }

    #[test]
    fn speaker_rename_applies_everywhere() {
        let mut t = Transcript::from_segments(
            "ru",
            10_000,
            vec![seg(0, 1000, "Раз", Some(0)), seg(1000, 2000, "Два", Some(1)), seg(2000, 3000, "Три", Some(0))],
        );
        assert!(t.rename_speaker(0, "  Иван "));
        assert!(!t.rename_speaker(5, "Никто"));
        assert!(!t.rename_speaker(1, "   "));
        let names: Vec<_> = t.paragraphs.iter().map(|p| t.speaker_name(p.speaker).unwrap()).collect();
        assert_eq!(names, vec!["Иван", "Спикер 2", "Иван"]);
        let txt = t.plain_text(true, false);
        assert!(txt.contains("Иван: Раз"));
        assert!(txt.contains("Иван: Три"));
        assert!(!txt.contains("Спикер 1"));
    }

    #[test]
    fn edit_marks_paragraph() {
        let mut t = Transcript::from_segments("ru", 1000, vec![seg(0, 1000, "Привет", None)]);
        assert!(t.set_paragraph_text(0, "Привет, мир"));
        assert!(t.paragraphs[0].edited);
        assert_eq!(t.plain_text(false, false), "Привет, мир\n");
    }

    #[test]
    fn drops_blank_markers() {
        let t = Transcript::from_segments("en", 1000, vec![seg(0, 1000, "[BLANK_AUDIO]", None)]);
        assert!(t.paragraphs.is_empty());
    }
}
