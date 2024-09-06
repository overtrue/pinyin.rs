mod error;
mod loader;
mod matcher;
mod pinyin;
use loader::WordsLoader;
use matcher::Matcher;

pub fn match_word_pinyin(word: &str) -> Vec<(String, String)> {
    let loader = WordsLoader::new();
    let matcher = Matcher::new(&loader);
    matcher
        .match_word_pinyin(word)
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
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
        assert_eq!(
            vec![
                "zhōng guó rén",
                "民",
                "xǐ huan",
                "在",
                "zhōng guó",
                "chī fàn",
                "，",
                "zhōng guó rén",
                "的",
                "kǒu wèi",
                "，",
                "zhōng guó",
                "饭",
                "hǎo chī"
            ],
            convert("中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃")
        );
    }
}
