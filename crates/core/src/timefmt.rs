//! Time formatting helpers.

/// «00:12:45» — timecode chip in the editor, DOCX and PDF.
pub fn chip(ms: u64) -> String {
    let s = ms / 1000;
    format!("{:02}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
}

/// «1:24:17» or «4:12» — human duration (recent files table).
pub fn duration(ms: u64) -> String {
    let s = (ms + 500) / 1000;
    if s >= 3600 {
        format!("{}:{:02}:{:02}", s / 3600, (s % 3600) / 60, s % 60)
    } else {
        format!("{:02}:{:02}", s / 60, s % 60)
    }
}

/// «00:01:02,345» — SRT timestamp.
pub fn srt(ms: u64) -> String {
    format!(
        "{:02}:{:02}:{:02},{:03}",
        ms / 3_600_000,
        (ms % 3_600_000) / 60_000,
        (ms % 60_000) / 1000,
        ms % 1000
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn formats() {
        assert_eq!(chip(765_400), "00:12:45");
        assert_eq!(chip(5_057_000), "01:24:17");
        assert_eq!(srt(3_723_045), "01:02:03,045");
        assert_eq!(srt(0), "00:00:00,000");
        assert_eq!(duration(252_000), "04:12");
        assert_eq!(duration(5_057_000), "1:24:17");
    }
}
