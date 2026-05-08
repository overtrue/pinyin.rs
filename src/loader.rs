use std::collections::{HashMap, HashSet};

const WORD_FILES: [&str; 10] = [
    include_str!(concat!(env!("OUT_DIR"), "/data/words_0.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_1.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_2.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_3.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_4.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_5.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_6.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_7.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_8.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/words_9.txt")),
];

const CHAR_FILES: [&str; 10] = [
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_0.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_1.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_2.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_3.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_4.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_5.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_6.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_7.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_8.txt")),
    include_str!(concat!(env!("OUT_DIR"), "/data/chars_9.txt")),
];

const SURNAMES: &str = include_str!(concat!(env!("OUT_DIR"), "/data/surnames.txt"));
const HETERONYMS: &str = include_str!(concat!(env!("OUT_DIR"), "/data/heteronyms.txt"));

#[derive(Debug, Default)]
pub(crate) struct Lexicon {
    words: HashMap<String, String>,
    chars: HashMap<String, String>,
    surnames: HashMap<String, String>,
    heteronyms: HashSet<char>,
}

impl Lexicon {
    pub(crate) fn new() -> Self {
        Self {
            words: load_map(WORD_FILES),
            chars: load_map(CHAR_FILES),
            surnames: load_map([SURNAMES]),
            heteronyms: HETERONYMS
                .lines()
                .filter_map(|line| line.chars().next())
                .collect(),
        }
    }

    pub(crate) fn default_entries(&self) -> Vec<(String, String)> {
        let mut entries = self.chars.clone();

        for (word, pinyin) in &self.surnames {
            entries.insert(word.clone(), pinyin.clone());
        }

        for (word, pinyin) in &self.words {
            entries.insert(word.clone(), pinyin.clone());
        }

        into_sorted_entries(entries)
    }

    pub(crate) fn plain_entries(&self) -> Vec<(String, String)> {
        let mut entries = self.chars.clone();

        for (word, pinyin) in &self.words {
            entries.insert(word.clone(), pinyin.clone());
        }

        into_sorted_entries(entries)
    }

    pub(crate) fn surname_entries(&self) -> Vec<(String, String)> {
        let mut entries = self.chars.clone();

        for (word, pinyin) in &self.words {
            entries.insert(word.clone(), pinyin.clone());
        }

        for (word, pinyin) in &self.surnames {
            entries.insert(word.clone(), pinyin.clone());
        }

        into_sorted_entries(entries)
    }

    pub(crate) fn char_pinyin(&self, ch: char) -> Option<&str> {
        self.chars.get(&ch.to_string()).map(String::as_str)
    }

    pub(crate) fn surname_pinyin(&self, text: &str) -> Option<&str> {
        self.surnames.get(text).map(String::as_str)
    }

    pub(crate) fn longest_surname_prefix<'a>(&self, input: &'a str) -> Option<&'a str> {
        self.surnames
            .keys()
            .filter_map(|surname| input.strip_prefix(surname).map(|_| surname.len()))
            .max()
            .map(|len| &input[..len])
    }

    pub(crate) fn heteronyms(&self, ch: char) -> Option<Vec<&str>> {
        if !self.heteronyms.contains(&ch) {
            return None;
        }

        self.char_pinyin(ch)
            .map(|pinyin| pinyin.split_whitespace().collect())
    }
}

fn load_map<const N: usize>(files: [&str; N]) -> HashMap<String, String> {
    files
        .into_iter()
        .flat_map(str::lines)
        .filter_map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let (key, value) = line.split_once(':')?;
    let key = key.trim();
    let value = value.trim();
    if key.is_empty() || value.is_empty() {
        return None;
    }

    Some((key.to_string(), value.to_string()))
}

fn into_sorted_entries(map: HashMap<String, String>) -> Vec<(String, String)> {
    let mut entries = map.into_iter().collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        right
            .0
            .chars()
            .count()
            .cmp(&left.0.chars().count())
            .then_with(|| left.0.cmp(&right.0))
    });
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_generated_rows() {
        assert_eq!(
            parse_line("中国: zhōng guó"),
            Some(("中国".to_string(), "zhōng guó".to_string()))
        );
        assert_eq!(parse_line("broken"), None);
    }

    #[test]
    fn loads_core_sources() {
        let lexicon = Lexicon::new();
        assert_eq!(lexicon.char_pinyin('中'), Some("zhōng zhòng"));
        assert_eq!(lexicon.surname_pinyin("单"), Some("shàn"));
        assert!(
            lexicon
                .default_entries()
                .iter()
                .any(|(word, _)| word == "你好")
        );
    }

    #[test]
    fn finds_longest_surname_prefix() {
        let lexicon = Lexicon::new();
        assert_eq!(lexicon.longest_surname_prefix("尉迟恭"), Some("尉迟"));
        assert_eq!(lexicon.longest_surname_prefix("张三"), None);
    }
}
