#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DictEntry {
    #[allow(dead_code)]
    pub word: String,
    #[serde(default)]
    pub phonetic: Option<String>,
    #[serde(default)]
    pub phonetics: Vec<DictPhonetic>,
    pub meanings: Vec<DictMeaning>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DictPhonetic {
    #[serde(default)]
    pub text: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    pub audio: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DictMeaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<DictDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DictDefinition {
    pub definition: String,
    #[serde(default)]
    pub example: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct MyMemoryResponse {
    #[serde(rename = "responseData")]
    response_data: MyMemoryData,
    #[allow(dead_code)]
    #[serde(default, rename = "responseStatus")]
    response_status: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
struct MyMemoryData {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

/// Final lookup result that combines Chinese translation and English
/// definitions, ready to be rendered in the side panel.
#[derive(Debug, Clone)]
pub struct DictResult {
    pub word: String,
    /// The Chinese translation (primary) – shown prominently at the top.
    pub chinese: String,
    /// Phonetic / IPA transcription (e.g. `/noʊ/`).
    pub phonetic: String,
    /// English definitions grouped by part of speech.
    pub sections: Vec<DictSection>,
    /// Stand-alone example sentences collected from the definitions.
    pub examples: Vec<String>,
    /// Set if neither the Chinese nor the English lookup returned anything.
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DictSection {
    pub part_of_speech: String,
    /// (definition, optional example)
    pub definitions: Vec<(String, Option<String>)>,
}

/// Look up an English word and return a [`DictResult`] that contains both a
/// Chinese translation (via MyMemory) and English definitions / examples
/// (via dictionaryapi.dev).
pub fn lookup(name: &str) -> DictResult {
    let chinese = lookup_chinese(name);
    let (phonetic, sections, examples) = lookup_english(name);
    let error = if chinese.is_empty() && sections.is_empty() {
        Some(format!("No definition found for \"{}\"", name))
    } else {
        None
    };
    DictResult {
        word: name.to_string(),
        chinese,
        phonetic,
        sections,
        examples,
        error,
    }
}

fn lookup_chinese(word: &str) -> String {
    let encoded: String = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", word)
        .append_pair("langpair", "en|zh-CN")
        .finish();
    let url = format!("https://api.mymemory.translated.net/get?{}", encoded);
    match ureq::get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .call()
    {
        Ok(resp) if resp.status() == 200 => match resp.into_string() {
            Ok(body) => match serde_json::from_str::<MyMemoryResponse>(&body) {
                Ok(data) => clean_translation(&data.response_data.translated_text),
                Err(_) => String::new(),
            },
            Err(_) => String::new(),
        },
        _ => String::new(),
    }
}

/// The MyMemory API echoes the input wrapped in uppercase messages such as
/// "MYMEMORY WARNING: ..." when the query is empty / out-of-vocabulary.
/// Strip those out so we only show a clean translation.
fn clean_translation(raw: &str) -> String {
    let s = raw.trim();
    if s.is_empty() {
        return String::new();
    }
    if s.contains("MYMEMORY WARNING") || s.contains("PLEASE SELECT TWO DISTINCT") {
        return String::new();
    }
    s.to_string()
}

fn lookup_english(word: &str) -> (String, Vec<DictSection>, Vec<String>) {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    match ureq::get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .call()
    {
        Ok(resp) if resp.status() == 200 => match resp.into_string() {
            Ok(body) => match serde_json::from_str::<Vec<DictEntry>>(&body) {
                Ok(entries) if !entries.is_empty() => extract_dict_info(&entries[0]),
                _ => (String::new(), Vec::new(), Vec::new()),
            },
            Err(_) => (String::new(), Vec::new(), Vec::new()),
        },
        Err(_) | Ok(_) => (String::new(), Vec::new(), Vec::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── clean_translation ───────────────────────────────────────

    #[test]
    fn test_clean_translation_normal() {
        assert_eq!(clean_translation("hello"), "hello");
    }

    #[test]
    fn test_clean_translation_empty() {
        assert_eq!(clean_translation(""), "");
        assert_eq!(clean_translation("   "), "");
    }

    #[test]
    fn test_clean_translation_mymemory_warning() {
        assert_eq!(
            clean_translation("MYMEMORY WARNING: YOU USED ALL AVAILABLE FREE TRANSLATIONS"),
            ""
        );
    }

    #[test]
    fn test_clean_translation_distinct_languages() {
        assert_eq!(
            clean_translation("PLEASE SELECT TWO DISTINCT LANGUAGES"),
            ""
        );
    }

    #[test]
    fn test_clean_translation_trims() {
        assert_eq!(clean_translation("  hello  "), "hello");
    }

    // ── extract_dict_info ───────────────────────────────────────

    fn make_entry(
        phonetic: Option<&str>,
        phonetics: Vec<&str>,
        meanings: Vec<(&str, Vec<(String, Option<String>)>)>,
    ) -> DictEntry {
        DictEntry {
            word: "test".to_string(),
            phonetic: phonetic.map(|s| s.to_string()),
            phonetics: phonetics
                .iter()
                .map(|t| DictPhonetic {
                    text: Some(t.to_string()),
                    audio: None,
                })
                .collect(),
            meanings: meanings
                .iter()
                .map(|(pos, defs)| DictMeaning {
                    part_of_speech: pos.to_string(),
                    definitions: defs
                        .iter()
                        .map(|(d, ex)| DictDefinition {
                            definition: d.clone(),
                            example: ex.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    #[test]
    fn test_extract_dict_info_with_phonetic() {
        let entry = make_entry(Some("/test/"), vec![], vec![]);
        let (phonetic, sections, examples) = extract_dict_info(&entry);
        assert_eq!(phonetic, "/test/");
        assert!(sections.is_empty());
        assert!(examples.is_empty());
    }

    #[test]
    fn test_extract_dict_info_phonetic_from_phonetics() {
        let entry = make_entry(None, vec!["/teɪk/"], vec![]);
        let (phonetic, _, _) = extract_dict_info(&entry);
        assert_eq!(phonetic, "/teɪk/");
    }

    #[test]
    fn test_extract_dict_info_prefers_phonetic_field() {
        let entry = make_entry(Some("/primary/"), vec!["/secondary/"], vec![]);
        let (phonetic, _, _) = extract_dict_info(&entry);
        assert_eq!(phonetic, "/primary/");
    }

    #[test]
    fn test_extract_dict_info_meanings() {
        let meanings = vec![(
            "noun",
            vec![
                ("A test".to_string(), None),
                ("Another test".to_string(), None),
            ],
        )];
        let entry = make_entry(None, vec![], meanings);
        let (_, sections, _) = extract_dict_info(&entry);
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].part_of_speech, "noun");
        assert_eq!(sections[0].definitions.len(), 2);
    }

    #[test]
    fn test_extract_dict_info_examples_deduplicated() {
        let meanings = vec![(
            "verb",
            vec![
                (
                    "Meaning 1".to_string(),
                    Some("Example sentence.".to_string()),
                ),
                (
                    "Meaning 2".to_string(),
                    Some("Example sentence.".to_string()),
                ),
            ],
        )];
        let entry = make_entry(None, vec![], meanings);
        let (_, _, examples) = extract_dict_info(&entry);
        assert_eq!(examples, vec!["Example sentence."]);
    }

    #[test]
    fn test_extract_dict_info_examples_max_five() {
        let meanings = vec![(
            "adj",
            (0..7)
                .map(|i| (format!("d{i}"), Some(format!("ex{i}"))))
                .collect::<Vec<_>>(),
        )];
        let entry = make_entry(None, vec![], meanings);
        let (_, _, examples) = extract_dict_info(&entry);
        assert!(examples.len() <= 5);
    }

    #[test]
    fn test_extract_dict_info_max_three_defs_per_pos() {
        let meanings = vec![(
            "noun",
            (0..5)
                .map(|i| (format!("def{i}"), None))
                .collect::<Vec<_>>(),
        )];
        let entry = make_entry(None, vec![], meanings);
        let (_, sections, _) = extract_dict_info(&entry);
        assert_eq!(sections[0].definitions.len(), 3);
    }

    #[test]
    fn test_extract_dict_info_empty() {
        let entry = make_entry(None, vec![], vec![]);
        let (phonetic, sections, examples) = extract_dict_info(&entry);
        assert_eq!(phonetic, "");
        assert!(sections.is_empty());
        assert!(examples.is_empty());
    }
}

fn extract_dict_info(e: &DictEntry) -> (String, Vec<DictSection>, Vec<String>) {
    let phonetic = e
        .phonetic
        .clone()
        .or_else(|| e.phonetics.iter().find_map(|p| p.text.clone()))
        .unwrap_or_default();
    let mut sections: Vec<DictSection> = Vec::new();
    let mut examples: Vec<String> = Vec::new();
    for m in &e.meanings {
        let mut defs: Vec<(String, Option<String>)> = Vec::new();
        for d in m.definitions.iter().take(3) {
            defs.push((d.definition.clone(), d.example.clone()));
            if let Some(ex) = &d.example {
                if examples.len() < 5 && !examples.iter().any(|x: &String| x == ex) {
                    examples.push(ex.clone());
                }
            }
        }
        sections.push(DictSection {
            part_of_speech: m.part_of_speech.clone(),
            definitions: defs,
        });
    }
    (phonetic, sections, examples)
}
