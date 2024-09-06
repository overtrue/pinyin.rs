mod error;
mod loader;
mod matcher;
mod pinyin;
use loader::{CharsLoader, SurnamesLoader, WordsLoader};
use matcher::Matcher;
use rayon::iter::*;
use std::sync::OnceLock;

// 已经线程安全
static WORDS_LOADER: OnceLock<WordsLoader> = OnceLock::new();
static SURNAMES_LOADER: OnceLock<SurnamesLoader> = OnceLock::new();
static CHARS_LOADER: OnceLock<CharsLoader> = OnceLock::new();
static MATCHERS: OnceLock<Vec<Matcher>> = OnceLock::new();

pub fn match_word_pinyin(word: &str) -> Vec<(String, String)> {
    let matchers = MATCHERS.get_or_init(|| {
        Vec::from([
            Matcher::new(WORDS_LOADER.get_or_init(WordsLoader::new)),
            Matcher::new(SURNAMES_LOADER.get_or_init(SurnamesLoader::new)),
            Matcher::new(CHARS_LOADER.get_or_init(CharsLoader::new)),
        ])
    });

    #[cfg(test)]
    let start = std::time::Instant::now();

    let mut results: Vec<_> = matchers
        .par_iter()
        .flat_map(|matcher| matcher.match_word_pinyin(word, false))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    results.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));

    #[cfg(test)]
    println!("match used: {}ms", start.elapsed().as_millis());

    results
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
    use crate::{convert, loader::WordsLoader, matcher::Matcher};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let cases = vec![
            (
                "中国人民喜欢在中国吃饭，中国人的口味，中国饭好吃",
                vec![
                    "zhōng guó rén",
                    "mín",
                    "xǐ huan",
                    "zài",
                    "zhōng guó",
                    "chī fàn",
                    "，",
                    "zhōng guó rén",
                    "dē",
                    "kǒu wèi",
                    "，",
                    "zhōng guó",
                    "fàn",
                    "hǎo chī",
                ],
            ),
            (
                "中国人喜欢中国吃饭",
                vec!["zhōng guó rén", "xǐ huan", "zhōng guó", "chī fàn"],
            ),
            ("四五六七", vec!["sì", "wǔ", "lù", "qī qí"]),
            (
                "尉迟恭大战单于丹",
                vec!["yù chí gōng", "dà zhàn", "chán yú", "dān"],
            ),
        ];
        for (input, want) in cases {
            assert_eq!(want, convert(input));
        }
    }

    #[test]
    fn test_matcher() {
        let start = std::time::Instant::now();
        let loader = WordsLoader::new();
        println!(
            "'DefaultLoader::new' used: {}ms",
            start.elapsed().as_millis()
        );

        let matcher = Matcher::new(&loader);

        let start = std::time::Instant::now();
        assert_eq!(vec!["nǐ hǎo", "，", "pì tī"], matcher.convert("你好，䴙䴘"));
        println!("'matcher.convert' used: {}ms", start.elapsed().as_millis());
    }
}
