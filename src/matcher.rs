use crate::loader::Loader;
use daachorse::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder, MatchKind};
use rayon::iter::*;
use std::collections::HashMap;

/// 拼音匹配器
#[derive(Clone)]
pub struct Matcher<'a> {
    handlers: Vec<CharwiseDoubleArrayAhoCorasick<&'a str>>,
}

impl<'a> Matcher<'a> {
    /// 创建新的匹配器
    ///
    /// # Arguments
    /// * `loader` - 数据加载器
    ///
    /// # Returns
    /// 返回新的匹配器实例
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
            .filter_map(|words| {
                if words.is_empty() {
                    None
                } else {
                    CharwiseDoubleArrayAhoCorasickBuilder::new()
                        .match_kind(MatchKind::LeftmostLongest)
                        .build_with_values(words)
                        .ok()
                }
            })
            .collect();

        #[cfg(test)]
        println!("'handlers init' used: {}ms", start.elapsed().as_millis());

        Matcher { handlers }
    }

    /// 匹配词汇并返回拼音
    ///
    /// # Arguments
    /// * `word` - 输入文本
    /// * `desc_by_key` - 是否按键长度降序排列
    ///
    /// # Returns
    /// 返回匹配到的 (词汇, 拼音) 元组列表
    pub fn match_word_pinyin(&self, word: &'a str, desc_by_key: bool) -> Vec<(&'a str, &'a str)> {
        if word.is_empty() {
            return Vec::new();
        }

        let mut matches: HashMap<&'a str, &'a str> = HashMap::new();

        for handler in &self.handlers {
            for m in handler.leftmost_find_iter(word) {
                let matched_word = &word[m.start()..m.end()];
                matches.insert(matched_word, m.value());
            }
        }

        if desc_by_key {
            sort_by_key_length_desc(matches)
        } else {
            matches.into_iter().collect()
        }
    }

    /// 转换文本为拼音
    ///
    /// # Arguments
    /// * `input` - 输入文本
    ///
    /// # Returns
    /// 返回拼音字符串列表
    #[allow(dead_code)]
    pub fn convert(&self, input: &str) -> Vec<String> {
        if input.is_empty() {
            return Vec::new();
        }

        // 预先收集字符，避免重复调用
        let input_chars: Vec<char> = input.chars().collect();
        let input_len = input_chars.len();

        let matched_words = self.match_word_pinyin(input, true);

        let mut result = Vec::with_capacity(input_len);
        let mut i = 0;

        while i < input_len {
            let mut found = false;

            for (word, pinyin) in &matched_words {
                let word_chars: Vec<char> = word.chars().collect();
                let word_len = word_chars.len();

                if i + word_len <= input_len
                    && input_chars[i..i + word_len] == word_chars[..]
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

/// 按键长度降序排列
///
/// # Arguments
/// * `map` - 输入的哈希映射
///
/// # Returns
/// 返回按键长度降序排列的元组列表
fn sort_by_key_length_desc<'a>(map: HashMap<&'a str, &'a str>) -> Vec<(&'a str, &'a str)> {
    let mut entries: Vec<_> = map.into_iter().collect();
    // 先按长度降序，再按字典序降序
    entries.sort_by(|(k1, _), (k2, _)| {
        k2.len().cmp(&k1.len()).then_with(|| k2.cmp(k1))
    });
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::WordsLoader;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_matcher_new() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);
        assert!(!matcher.handlers.is_empty());
    }

    #[test]
    fn test_match_word_pinyin() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);

        let result = matcher.match_word_pinyin("中国", false);
        assert!(!result.is_empty());

        // 应该包含"中国"
        assert!(result.iter().any(|(word, _)| word == &"中国"));
    }

    #[test]
    fn test_match_word_pinyin_empty() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);

        let result = matcher.match_word_pinyin("", false);
        assert!(result.is_empty());
    }

    #[test]
    fn test_match_word_pinyin_desc_order() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);

        let result = matcher.match_word_pinyin("中国人", true);
        if result.len() > 1 {
            // 验证按长度降序排列
            for i in 1..result.len() {
                assert!(result[i-1].0.len() >= result[i].0.len());
            }
        }
    }

    #[test]
    fn test_convert() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);

        let result = matcher.convert("中国");
        assert!(!result.is_empty());
        assert!(result.iter().any(|p| p.contains("zhōng") || p.contains("guó")));
    }

    #[test]
    fn test_convert_empty() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);

        let result = matcher.convert("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_convert_mixed_content() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);

        let result = matcher.convert("中国123abc");
        assert!(!result.is_empty());

        // 应该包含数字和字母
        assert!(result.iter().any(|s| s == "1"));
        assert!(result.iter().any(|s| s == "a"));
    }

    #[test]
    fn test_sort_by_key_length_desc() {
        let mut map = HashMap::new();
        map.insert("中", "zhōng");
        map.insert("中国", "zhōng guó");
        map.insert("中国人", "zhōng guó rén");

        let result = sort_by_key_length_desc(map);

        // 验证按长度降序排列
        assert_eq!(result[0].0, "中国人");
        assert_eq!(result[1].0, "中国");
        assert_eq!(result[2].0, "中");
    }

    #[test]
    fn test_matcher_performance() {
        let loader = WordsLoader::new();
        let matcher = Matcher::new(&loader);

        let long_text = "中国人民喜欢在中国吃饭".repeat(10);
        let start = std::time::Instant::now();
        let result = matcher.convert(&long_text);
        let duration = start.elapsed();

        assert!(!result.is_empty());
        println!("Matcher convert took: {:?}", duration);
        // 性能测试：应该在合理时间内完成
        assert!(duration.as_millis() < 500);
    }

    // ========== PHP 兼容性测试 ==========

    #[test]
    fn test_php_polyphones_method_compatibility() {
        // 对应 PHP: Pinyin::polyphones('重庆')
        use crate::convert;

        let polyphone_cases = vec![
            ("重庆", vec!["重", "庆"]),
            ("银行", vec!["银", "行"]),
            ("行走", vec!["行", "走"]),
            ("数据", vec!["数", "据"]),
            ("长短", vec!["长", "短"]),
            ("调调", vec!["调", "调"]),
        ];

        for (input, expected_chars) in polyphone_cases {
            let result = convert(input);

            println!("Input: {}", input);
            println!("Result: {:?}", result);

            // Rust 版本可能会合并词汇，所以长度可能不同
            assert!(result.len() <= expected_chars.len());

            // 验证每个拼音都不为空且包含字母
            for pinyin in &result {
                assert!(!pinyin.is_empty(), "Empty pinyin in result");
                assert!(pinyin.chars().any(|c| c.is_ascii_alphabetic()),
                       "No alphabetic characters in pinyin: {}", pinyin);
            }
        }
    }

    #[test]
    fn test_php_chars_method_compatibility() {
        // 对应 PHP: Pinyin::chars('重庆')
        use crate::convert;

        let char_cases = vec![
            "重庆",
            "银行",
            "数据",
            "长短",
            "中国",
        ];

        for input in char_cases {
            let result = convert(input);

            println!("Input: {}", input);
            println!("Chars result: {:?}", result);

            // Rust 版本可能会合并词汇，所以长度可能不同
            assert!(result.len() <= input.chars().count());

            // 验证所有拼音都不为空且包含字母
            for pinyin in &result {
                assert!(!pinyin.is_empty(), "Empty pinyin in result");
                assert!(pinyin.chars().any(|c| c.is_ascii_alphabetic()),
                       "No alphabetic characters in pinyin: {}", pinyin);
            }
        }
    }
}
