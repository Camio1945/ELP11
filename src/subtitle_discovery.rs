/// Search the same directory as `video_path` for an external subtitle file
/// whose name starts with the video's stem.  When multiple candidates exist,
/// the one **without** a language suffix (e.g. `.zh-CN`) is preferred as the
/// default / English subtitle.
pub fn find_english_subtitle_file(video_path: &str) -> Option<std::path::PathBuf> {
    let video_path = std::path::Path::new(video_path);
    let dir = video_path.parent()?;
    let stem = video_path.file_stem()?.to_str()?;

    let mut candidates = collect_subtitle_candidates(dir, stem)?;

    if candidates.is_empty() {
        return None;
    }

    pick_best_subtitle_candidate(&mut candidates)
}

fn collect_subtitle_candidates(
    dir: &std::path::Path,
    stem: &str,
) -> Option<Vec<(std::path::PathBuf, String)>> {
    let subtitle_exts = ["srt", "ass", "ssa", "vtt", "sub", "smi"];
    let mut candidates: Vec<(std::path::PathBuf, String)> = Vec::new();

    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let Some(remainder) = name.strip_prefix(stem) else {
            continue;
        };
        if let Some(ext) = path.extension() {
            let ext_lower = ext.to_str()?.to_lowercase();
            if subtitle_exts.contains(&ext_lower.as_str()) {
                candidates.push((path.clone(), remainder.to_string()));
            }
        }
    }
    Some(candidates)
}

/// Prefer the default/English subtitle: first the one whose remainder is
/// only the extension (e.g. ".srt" → empty), then one with an explicit
/// English suffix (".en", ".eng", ".en-US"...), then the first candidate.
fn pick_best_subtitle_candidate(
    candidates: &mut Vec<(std::path::PathBuf, String)>,
) -> Option<std::path::PathBuf> {
    for (path, remainder) in candidates.iter() {
        if strip_extension(remainder).is_empty() {
            return Some(path.clone());
        }
    }
    for (path, remainder) in candidates.iter() {
        if is_english_suffix(strip_extension(remainder)) {
            return Some(path.clone());
        }
    }
    // Fallback: return the first candidate
    Some(candidates.remove(0).0)
}

/// Remove the subtitle extension from the remainder, e.g. ".zh-CN.srt" → ".zh-CN".
fn strip_extension(remainder: &str) -> &str {
    if let Some(dot_pos) = remainder.rfind('.') {
        &remainder[..dot_pos]
    } else {
        remainder
    }
}

/// Whether a remainder suffix marks the subtitle as English, e.g. ".en".
fn is_english_suffix(suffix_no_ext: &str) -> bool {
    let s = suffix_no_ext
        .trim_start_matches('.')
        .to_lowercase()
        .replace('_', "-");
    s == "en" || s == "eng" || s == "english" || s.starts_with("en-") || s.starts_with("eng-")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── strip_extension ─────────────────────────────────────────

    #[test]
    fn test_strip_extension_with_ext() {
        assert_eq!(strip_extension(".en.srt"), ".en");
        assert_eq!(strip_extension(".zh-CN.srt"), ".zh-CN");
        assert_eq!(strip_extension(".srt"), "");
    }

    #[test]
    fn test_strip_extension_no_ext() {
        assert_eq!(strip_extension("noext"), "noext");
    }

    #[test]
    fn test_strip_extension_no_dot() {
        assert_eq!(strip_extension(".en"), "");
    }

    // ── is_english_suffix ───────────────────────────────────────

    #[test]
    fn test_is_english_suffix_en() {
        assert!(is_english_suffix(".en"));
        assert!(is_english_suffix("en"));
    }

    #[test]
    fn test_is_english_suffix_eng() {
        assert!(is_english_suffix(".eng"));
        assert!(is_english_suffix("eng"));
    }

    #[test]
    fn test_is_english_suffix_full() {
        assert!(is_english_suffix(".english"));
    }

    #[test]
    fn test_is_english_suffix_with_region() {
        assert!(is_english_suffix(".en-US"));
        assert!(is_english_suffix(".en-GB"));
        assert!(is_english_suffix(".eng-US"));
    }

    #[test]
    fn test_is_english_suffix_underscore() {
        assert!(is_english_suffix(".en_US"));
    }

    #[test]
    fn test_is_english_suffix_case_insensitive() {
        assert!(is_english_suffix(".EN"));
        assert!(is_english_suffix(".En-Us"));
    }

    #[test]
    fn test_is_english_suffix_non_english() {
        assert!(!is_english_suffix(".zh-CN"));
        assert!(!is_english_suffix(".ja"));
        assert!(!is_english_suffix(".fr"));
        assert!(!is_english_suffix(".ko"));
    }

    // ── pick_best_subtitle_candidate ────────────────────────────

    fn make_candidate(path: &str, remainder: &str) -> (std::path::PathBuf, String) {
        (std::path::PathBuf::from(path), remainder.to_string())
    }

    #[test]
    fn test_pick_best_prefers_default() {
        let mut candidates = vec![
            make_candidate("movie.en.srt", ".en.srt"),
            make_candidate("movie.srt", ".srt"), // default — preferred
        ];
        let result = pick_best_subtitle_candidate(&mut candidates).unwrap();
        assert!(result.to_str().unwrap().ends_with("movie.srt"));
    }

    #[test]
    fn test_pick_best_falls_back_to_english() {
        let mut candidates = vec![
            make_candidate("movie.zh-CN.srt", ".zh-CN.srt"),
            make_candidate("movie.en.srt", ".en.srt"),
        ];
        let result = pick_best_subtitle_candidate(&mut candidates).unwrap();
        assert!(result.to_str().unwrap().ends_with("movie.en.srt"));
    }

    #[test]
    fn test_pick_best_fallback_first() {
        let mut candidates = vec![
            make_candidate("movie.zh-CN.srt", ".zh-CN.srt"),
            make_candidate("movie.ja.srt", ".ja.srt"),
        ];
        let result = pick_best_subtitle_candidate(&mut candidates).unwrap();
        // Falls back to first candidate since neither is default nor English
        assert!(result.to_str().unwrap().ends_with("movie.zh-CN.srt"));
    }
}
