pub fn format_time(secs: f64) -> String {
    let total = secs as u64;
    let h = total / 3600;
    let m = (total % 3600) / 60;
    let s = total % 60;
    if h > 0 {
        format!("{}:{:02}:{:02}", h, m, s)
    } else {
        format!("{}:{:02}", m, s)
    }
}

pub fn clean_subtitle_text(raw: &str) -> String {
    let mut s = raw.to_string();
    s = s
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<u>", "")
        .replace("</u>", "")
        .replace("<font", "X")
        .replace("</font>", "")
        .replace("\\N", "\n")
        .replace("\\n", "\n")
        .replace("\\h", " ");
    s = s
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
        .replace("&#34;", "\"")
        .replace("&amp;", "&")
        .replace("&#38;", "&")
        .replace("&lt;", "<")
        .replace("&#60;", "<")
        .replace("&gt;", ">")
        .replace("&#62;", ">")
        .replace("&nbsp;", " ")
        .replace("&#160;", " ");
    s.trim().to_string()
}

/// Stop words kept for potential future use (e.g. excluding common words
/// from vocabulary extraction).  All subtitle words are currently
/// clickable, so this list is no longer consulted at render time.
#[allow(dead_code)]
pub const STOP_WORDS: &[&str] = &[
    "a", "an", "the", "and", "or", "but", "if", "then", "else", "when", "at", "by", "for", "from",
    "in", "of", "on", "to", "with", "as", "is", "was", "are", "were", "be", "been", "being", "am",
    "do", "does", "did", "done", "have", "has", "had", "having", "will", "would", "should",
    "could", "can", "may", "might", "must", "shall", "it", "its", "this", "that", "these", "those",
    "i", "me", "my", "we", "us", "our", "you", "your", "he", "him", "his", "she", "her", "they",
    "them", "their", "what", "which", "who", "whom", "whose", "not", "no", "nor", "so", "too",
    "very", "just", "up", "down", "out", "about", "into", "over", "after", "before", "between",
    "through", "during", "above", "below", "re", "ve", "ll", "s", "t", "don", "didn", "doesn",
    "won", "isn", "aren", "couldn", "shouldn", "wouldn", "wasn", "weren", "hasn", "haven", "hadn",
    "mustn", "mightn", "apos", "ndash", "quot", "amp", "lt", "gt",
];

#[cfg(test)]
mod tests {
    use super::*;

    // ── format_time ─────────────────────────────────────────────

    #[test]
    fn test_format_time_zero() {
        assert_eq!(format_time(0.0), "0:00");
    }

    #[test]
    fn test_format_time_seconds_only() {
        assert_eq!(format_time(5.0), "0:05");
        assert_eq!(format_time(45.5), "0:45");
    }

    #[test]
    fn test_format_time_minutes() {
        assert_eq!(format_time(65.0), "1:05");
        assert_eq!(format_time(599.0), "9:59");
    }

    #[test]
    fn test_format_time_hours() {
        assert_eq!(format_time(3600.0), "1:00:00");
        assert_eq!(format_time(3661.0), "1:01:01");
        assert_eq!(format_time(45296.0), "12:34:56");
    }

    #[test]
    fn test_format_time_fractional() {
        // Fractional seconds should be truncated by u64 cast
        assert_eq!(format_time(10.9), "0:10");
    }

    // ── clean_subtitle_text ─────────────────────────────────────

    #[test]
    fn test_clean_subtitle_text_bold_tags() {
        assert_eq!(clean_subtitle_text("<b>Hello</b>"), "Hello");
    }

    #[test]
    fn test_clean_subtitle_text_italic_tags() {
        assert_eq!(clean_subtitle_text("<i>World</i>"), "World");
    }

    #[test]
    fn test_clean_subtitle_text_underline_tags() {
        assert_eq!(clean_subtitle_text("<u>Text</u>"), "Text");
    }

    #[test]
    fn test_clean_subtitle_text_font_tag() {
        // <font is replaced with X, </font> is removed
        let result = clean_subtitle_text("<font color=\"red\">Colored</font>");
        assert!(result.contains("X") || result.contains("Colored"));
    }

    #[test]
    fn test_clean_subtitle_text_backslash_sequences() {
        assert_eq!(clean_subtitle_text("Line1\\NLine2"), "Line1\nLine2");
        assert_eq!(clean_subtitle_text("A\\nB"), "A\nB");
        assert_eq!(clean_subtitle_text("Hello\\hWorld"), "Hello World");
    }

    #[test]
    fn test_clean_subtitle_text_html_entities() {
        assert_eq!(clean_subtitle_text("It&apos;s"), "It's");
        assert_eq!(clean_subtitle_text("&#39;"), "'");
        assert_eq!(clean_subtitle_text("&quot;quote&quot;"), "\"quote\"");
        assert_eq!(clean_subtitle_text("&#34;"), "\"");
        assert_eq!(clean_subtitle_text("A &amp; B"), "A & B");
        assert_eq!(clean_subtitle_text("&#38;"), "&");
        assert_eq!(clean_subtitle_text("x &lt; 5"), "x < 5");
        assert_eq!(clean_subtitle_text("y &gt; 3"), "y > 3");
        // &nbsp; is replaced with space, then trim() removes surrounding whitespace
        assert_eq!(clean_subtitle_text("hello&nbsp;world"), "hello world");
        assert_eq!(clean_subtitle_text("&#160;hello"), "hello");
    }

    #[test]
    fn test_clean_subtitle_text_trim() {
        assert_eq!(clean_subtitle_text("  hello  "), "hello");
        assert_eq!(clean_subtitle_text("\n  text  \n"), "text");
    }

    #[test]
    fn test_clean_subtitle_text_unchanged() {
        assert_eq!(
            clean_subtitle_text("Plain text with no tags."),
            "Plain text with no tags."
        );
    }

    #[test]
    fn test_clean_subtitle_text_mixed() {
        let input = "<b>Hello</b> &amp; <i>world</i>\\NWelcome";
        let result = clean_subtitle_text(input);
        assert!(result.contains("Hello"));
        assert!(result.contains("&"));
        assert!(result.contains("world"));
        assert!(result.contains("Welcome"));
    }
}
