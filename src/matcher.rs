use crate::loader::{CharsLoader, Loader, SurnamesLoader, WordsLoader};
use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use rayon::iter::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::OnceLock;
use crate::PinyinWord;

#[derive(Clone)]
pub struct Matcher<'a> {
    handlers: Vec<CharwiseDoubleArrayAhoCorasick<&'a str>>,
}

#[derive(Debug, PartialEq)]
pub enum MatchSegment {
    Matched(PinyinWord),
    NotMatched(char),
}

impl<'a> Matcher<'a> {
    pub fn new<L: Loader>(loader: &'a L) -> Self {
        #[cfg(test)]
        let start = std::time::Instant::now();
        let words = loader.load();

        #[cfg(test)]
        println!("'get_chunk_words' used: {}ms", start.elapsed().as_millis());

        #[cfg(test)]
        let start = std::time::Instant::now();

        let handlers = words
            .into_par_iter()
            .map(|words| {
                CharwiseDoubleArrayAhoCorasickBuilder::new()
                    .match_kind(MatchKind::LeftmostLongest)
                    .build_with_values(words)
                    .unwrap()
            })
            .collect();

        #[cfg(test)]
        println!("'handlers init' used: {}ms", start.elapsed().as_millis());

        Matcher { handlers }
    }

    pub fn match_to_pinyin(&self, word: &'a str, desc_by_key: bool) -> Vec<(&'a str, &'a str)> {
        let iter = self.handlers.iter().flat_map(|handler| {
            handler
                .leftmost_find_iter(word)
                .map(|m| {
                    let matched_word = &word[m.start()..m.end()];
                    (matched_word, m.value())
                })
                .collect::<HashMap<&'a str, &'a str>>()
        });

        if desc_by_key {
            return sort_by_key_length_desc(iter.collect());
        }

        iter.collect()
    }
}

fn sort_by_key_length_desc<'a>(map: HashMap<&'a str, &'a str>) -> Vec<(&'a str, &'a str)> {
    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));
    entries
}


// 已经线程安全
static WORDS_LOADER: OnceLock<WordsLoader> = OnceLock::new();
static SURNAMES_LOADER: OnceLock<SurnamesLoader> = OnceLock::new();
static CHARS_LOADER: OnceLock<CharsLoader> = OnceLock::new();
static WORD_MATCHERS: OnceLock<Vec<Matcher>> = OnceLock::new();

pub fn match_char_pinyin(ch: char) -> Vec<(String, String)> {
    #[cfg(test)]
    let start = std::time::Instant::now();
    let matcher = Matcher::new(CHARS_LOADER.get_or_init(|| CharsLoader::new()));

    let mut results: Vec<_> = matcher
        .match_to_pinyin(&ch.to_string(), false)
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    results.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));

    #[cfg(test)]
    println!("match char used: {}ms", start.elapsed().as_millis());

    results
}

pub fn match_word_pinyin(word: &str) -> Vec<(String, String)> {
    let matchers = WORD_MATCHERS.get_or_init(|| {
        vec![
            Matcher::new(WORDS_LOADER.get_or_init(|| WordsLoader::new())),
            Matcher::new(CHARS_LOADER.get_or_init(|| CharsLoader::new())),
        ]
    });

    #[cfg(test)]
    let start = std::time::Instant::now();

    let mut results: Vec<_> = matchers
        .par_iter()
        .flat_map(|matcher| matcher.match_to_pinyin(word, true))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    #[cfg(test)]
    println!("match words used: {}ms", start.elapsed().as_millis());

    results
}

pub fn match_surname_pinyin(surname: &str) -> Vec<(String, String)> {
    #[cfg(test)]
    let start = std::time::Instant::now();
    let matcher = Matcher::new(SURNAMES_LOADER.get_or_init(|| SurnamesLoader::new()));
    // 把姓名两部分，姓和名，分开
    let last_name = &surname.chars().take(2).collect::<String>();

    let mut results: Vec<_> = matcher
        .match_to_pinyin(last_name, true)
        .into_iter().map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    // 虽然取了两个字符，但是可能姓只有一个字，名有多个字，所以这里按结果长度截取
    let first_name = &surname.chars().skip(results[0].0.chars().count()).collect::<String>();

    // 名字部分用 match_word_pinyin 匹配
    if !first_name.is_empty() {
        let mut first_name_results: Vec<_> = match_word_pinyin(first_name).into_iter().collect();
        results.append(&mut first_name_results);
    }

    #[cfg(test)]
    println!("match surname used: {}ms", start.elapsed().as_millis());

    results
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_match_word_pinyin() {
        let start = std::time::Instant::now();
        println!(
            "'DefaultLoader::new' used: {}ms",
            start.elapsed().as_millis()
        );

        let start = std::time::Instant::now();
        let result = super::match_word_pinyin("你好");
        println!("'match_word_pinyin' used: {}ms", start.elapsed().as_millis());
        assert_eq!(vec![
            ("你好".to_string(), "nǐ hǎo".to_string()),
            ("好".to_string(), "hǎo hào".to_string()),
            ("你".to_string(), "nǐ".to_string()),
        ], result);
    }

    #[test]
    fn test_match_world_with_symbol() {
        let start = std::time::Instant::now();
        let result = super::match_word_pinyin("你好，世界！");
        println!("'match_word_pinyin' used: {}ms", start.elapsed().as_millis());
        assert_eq!(vec![
            ("你好".to_string(), "nǐ hǎo".to_string()),
            ("世界".to_string(), "shì jiè".to_string()),
            ("界".to_string(), "jiè".to_string()),
            ("好".to_string(), "hǎo hào".to_string()),
            ("你".to_string(), "nǐ".to_string()),
            ("世".to_string(), "shì".to_string()),
        ], result);
    }

    #[test]
    fn test_match_surname_pinyin() {
        let start = std::time::Instant::now();
        println!(
            "'DefaultLoader::new' used: {}ms",
            start.elapsed().as_millis()
        );

        let start = std::time::Instant::now();
        let result = super::match_surname_pinyin("尉迟恭");
        println!("'match_surnames_pinyin' used: {}ms", start.elapsed().as_millis());
        assert_eq!(vec![("尉迟".to_string(), "yù chí".to_string()), ("恭".to_string(), "gōng".to_string())], result);
    }

    #[test]
    fn test_match_char_pinyin() {
        let start = std::time::Instant::now();
        let result = super::match_char_pinyin('你');
        println!("'match_char_pinyin' used: {}ms", start.elapsed().as_millis());
        assert_eq!(vec![("你".to_string(), "nǐ".to_string())], result);
    }
}