mod error;
mod pinyin;
use daachorse::{CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use std::collections::HashMap;

fn sort_by_key_length_desc(map: HashMap<String, String>) -> Vec<(String, String)> {
    let mut entries = map.into_iter().collect::<Vec<_>>();
    entries.sort_by(|(k1, _), (k2, _)| k2.len().cmp(&k1.len()));
    entries
}

pub fn match_word_pinyin(word: &str) -> Vec<(String, String)> {
    let words = vec![
        ("中国", "zhong guo1"),
        ("中国人", "zhong guo ren2"),
        ("中国人民", "zhong guo ren min3"),
    ];
    let pma = CharwiseDoubleArrayAhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostLongest)
        .build_with_values(words)
        .unwrap();
    let mut result = HashMap::new();

    let it = pma.leftmost_find_iter(word);

    for m in it {
        let matched_word = &word[m.start()..m.end()];
        result.insert(matched_word.to_string(), m.value().to_string());
    }

    sort_by_key_length_desc(result)
}

pub fn convert(input: &str) -> Vec<String> {
    // 先把整句话拿去匹配全部命中的词
    let input_len = input.chars().count();
    let matched_words = match_word_pinyin(input);
    let input_chars = input.chars().collect::<Vec<_>>();

    let mut result = Vec::new();
    let mut i = 0;

    while i < input_len {
        let mut found = false;
        for (word, pinyin) in &matched_words {
            let word_len = word.chars().count();
            if i + word_len <= input_len
                && &input_chars[i..i + word_len] == word.chars().collect::<Vec<_>>().as_slice()
            {
                result.push(pinyin.clone());
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
        print!(
            "{:?}",
            convert("中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃")
        );
    }
}
