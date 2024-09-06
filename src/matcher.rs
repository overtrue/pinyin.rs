use crate::loader::Loader;
use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use rayon::iter::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Matcher<'a> {
    handlers: Vec<CharwiseDoubleArrayAhoCorasick<&'a str>>,
}

impl<'a> Matcher<'a> {
    pub fn new<L: Loader>(loader: &'a L) -> Self {
        #[cfg(test)]
        let start = std::time::Instant::now();

        let words = loader.get_chunks(11);
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

    pub fn match_word_pinyin(&self, word: &'a str, desc_by_key: bool) -> Vec<(&'a str, &'a str)> {
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

    #[allow(dead_code)]
    pub fn convert(&self, input: &str) -> Vec<String> {
        // 先把整句话拿去匹配全部命中的词
        let input_len = input.chars().count();

        let matched_words = self.match_word_pinyin(input, true);

        let input_chars: Vec<char> = input.chars().collect();

        let mut result = Vec::new();
        let mut i = 0;

        while i < input_len {
            let mut found = false;
            for (word, pinyin) in matched_words.iter() {
                let word_len = word.chars().count();
                if i + word_len <= input_len
                    && &input_chars[i..i + word_len] == word.chars().collect::<Vec<_>>().as_slice()
                {
                    result.push(pinyin.to_string());
                    i += word_len;
                    found = true;
                    break;
                }
            }

            if !found {
                result.push(input_chars[i].to_string());
                i += 1;
            }
        }

        result
    }
}

fn sort_by_key_length_desc<'a>(map: HashMap<&'a str, &'a str>) -> Vec<(&'a str, &'a str)> {
    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));
    entries
}
