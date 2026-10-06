use crate::transcript::Transcript;

/// Plain text: paragraphs separated by blank lines; «Имя:» prefix when speakers are known.
pub fn to_txt(t: &Transcript) -> String {
    t.plain_text(t.has_speakers(), false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn txt_cyrillic_utf8() {
        let t = crate::export::testdata::sample();
        let s = to_txt(&t);
        assert!(s.starts_with("Иван: Мы сегодня обсуждаем"));
        assert!(s.contains("\n\nСпикер 2: Да, согласен."));
        assert!(s.contains("«Ёлка» №1"));
        // valid UTF-8 round trip
        let bytes = s.clone().into_bytes();
        assert_eq!(String::from_utf8(bytes).unwrap(), s);
    }
}
