mod error;
mod pinyin;
use daachorse::{CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use std::collections::HashMap;

// import by compiler
include!(concat!(env!("OUT_DIR"), "/__words.rs"));

fn sort_by_key_length_desc<'a>(map: HashMap<&'a str, &'a str>) -> Vec<(&'a str, &'a str)> {
    let mut entries: Vec<_> = map.into_iter().collect();
    entries.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));
    entries
}

pub fn match_word_pinyin(word: &str) -> Vec<(String, String)> {
    let words = INCLUDE_WORDS;
    let pma = CharwiseDoubleArrayAhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build_with_values(words)
        .unwrap();
    let result = pma
        .leftmost_find_iter(word)
        .map(|m| {
            let matched_word = &word[m.start()..m.end()];
            (matched_word, m.value())
        })
        .collect();

    sort_by_key_length_desc(result)
}

pub fn convert(input: &str) -> Vec<String> {
    // 先把整句话拿去匹配全部命中的词
    let input_len = input.chars().count();
    let matched_words = match_word_pinyin(input);
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

#[cfg(test)]
mod tests {
    use crate::convert;

    #[test]
    fn it_works() {
        println!("{:?}", super::INCLUDE_WORDS);
        assert_eq!(
            vec![
                "zhōng guó rén mén",
                "喜",
                "欢",
                "在",
                "zhōng guó",
                "吃",
                "饭",
                "，",
                "zhōng guó rén",
                "的",
                "口",
                "味",
                "，",
                "zhōng guó",
                "饭",
                "好",
                "吃"
            ],
            convert("中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃")
        );
    }
}
