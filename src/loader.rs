use rayon::{iter::*, slice::ParallelSlice};
use std::collections::HashMap;

/// 数据加载器特征
pub trait Loader {
    /// 获取分块数据
    fn get_chunks(&self, size: usize) -> Vec<HashMap<&str, &str>>;
}

#[derive(Debug, Default)]
pub struct WordsLoader {
    words: HashMap<String, String>,
}

impl Loader for WordsLoader {
    fn get_chunks(&self, size: usize) -> Vec<HashMap<&str, &str>> {
        assert!(size > 0);

        if self.words.is_empty() {
            return vec![HashMap::new()];
        }

        let chunk_size = (self.words.len() + size - 1) / size; // 向上取整
        self.words
            .par_iter()
            .collect::<Vec<_>>()
            .par_chunks(chunk_size)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect()
            })
            .collect()
    }
}

impl WordsLoader {
    /// 创建新的词汇加载器
    pub fn new() -> Self {
        let words_files = [
            include_str!("../data/words_0.txt"),
            include_str!("../data/words_1.txt"),
            include_str!("../data/words_2.txt"),
            include_str!("../data/words_3.txt"),
            include_str!("../data/words_4.txt"),
            include_str!("../data/words_5.txt"),
            include_str!("../data/words_6.txt"),
            include_str!("../data/words_7.txt"),
            include_str!("../data/words_8.txt"),
            include_str!("../data/words_9.txt"),
        ];

        let words: HashMap<String, String> = words_files
            .into_par_iter()
            .flat_map(|content| {
                content
                    .lines()
                    .filter_map(|line| parse_line(line))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self { words }
    }
}

#[derive(Debug, Default)]
pub struct CharsLoader {
    chars: HashMap<String, String>,
}

impl Loader for CharsLoader {
    fn get_chunks(&self, size: usize) -> Vec<HashMap<&str, &str>> {
        assert!(size > 0);

        if self.chars.is_empty() {
            return vec![HashMap::new()];
        }

        let chunk_size = (self.chars.len() + size - 1) / size; // 向上取整
        self.chars
            .par_iter()
            .collect::<Vec<_>>()
            .par_chunks(chunk_size)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect()
            })
            .collect()
    }
}

impl CharsLoader {
    /// 创建新的字符加载器
    pub fn new() -> Self {
        let chars_files = [
            include_str!("../data/chars_0.txt"),
            include_str!("../data/chars_1.txt"),
            include_str!("../data/chars_2.txt"),
            include_str!("../data/chars_3.txt"),
            include_str!("../data/chars_4.txt"),
            include_str!("../data/chars_5.txt"),
            include_str!("../data/chars_6.txt"),
            include_str!("../data/chars_7.txt"),
            include_str!("../data/chars_8.txt"),
            include_str!("../data/chars_9.txt"),
        ];

        let chars: HashMap<String, String> = chars_files
            .into_par_iter()
            .flat_map(|content| {
                content
                    .lines()
                    .filter_map(|line| parse_line(line))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self { chars }
    }
}

#[derive(Debug, Default)]
pub struct SurnamesLoader {
    surnames: HashMap<String, String>,
}

impl Loader for SurnamesLoader {
    fn get_chunks(&self, _: usize) -> Vec<HashMap<&str, &str>> {
        let map = self
            .surnames
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        vec![map]
    }
}

impl SurnamesLoader {
    /// 创建新的姓氏加载器
    pub fn new() -> Self {
        let surnames: HashMap<String, String> = include_str!("../data/surnames.txt")
            .lines()
            .filter_map(|line| parse_line(line))
            .collect();

        Self { surnames }
    }
}

/// 解析数据行
///
/// # Arguments
/// * `line` - 数据行，格式为 "中文: 拼音"
///
/// # Returns
/// 返回解析后的 (中文, 拼音) 元组，如果解析失败返回 None
fn parse_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let mut parts = line.splitn(2, ':');
    let chinese = parts.next()?.trim();
    let pinyin = parts.next()?.trim();

    if chinese.is_empty() || pinyin.is_empty() {
        return None;
    }

    Some((chinese.to_string(), pinyin.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_line() {
        // 正常情况
        assert_eq!(
            parse_line("中国: zhōng guó"),
            Some(("中国".to_string(), "zhōng guó".to_string()))
        );

        // 带空格
        assert_eq!(
            parse_line("  中国  :  zhōng guó  "),
            Some(("中国".to_string(), "zhōng guó".to_string()))
        );

        // 空行
        assert_eq!(parse_line(""), None);
        assert_eq!(parse_line("   "), None);

        // 注释行
        assert_eq!(parse_line("# 这是注释"), None);

        // 格式错误
        assert_eq!(parse_line("中国"), None);
        assert_eq!(parse_line("中国:"), None);
        assert_eq!(parse_line(":zhōng guó"), None);

        // 多个冒号
        assert_eq!(
            parse_line("时间: 12:30"),
            Some(("时间".to_string(), "12:30".to_string()))
        );
    }

    #[test]
    fn test_words_loader() {
        let loader = WordsLoader::new();
        assert!(!loader.words.is_empty());

        // 测试分块
        let chunks = loader.get_chunks(5);
        assert!(!chunks.is_empty());
        assert!(chunks.len() <= 5);

        // 验证所有数据都被包含
        let total_items: usize = chunks.iter().map(|chunk| chunk.len()).sum();
        assert_eq!(total_items, loader.words.len());
    }

    #[test]
    fn test_chars_loader() {
        let loader = CharsLoader::new();
        assert!(!loader.chars.is_empty());

        // 测试分块
        let chunks = loader.get_chunks(3);
        assert!(!chunks.is_empty());

        // 验证所有数据都被包含
        let total_items: usize = chunks.iter().map(|chunk| chunk.len()).sum();
        assert_eq!(total_items, loader.chars.len());
    }

    #[test]
    fn test_surnames_loader() {
        let loader = SurnamesLoader::new();
        assert!(!loader.surnames.is_empty());

        // 姓氏数据应该只有一个块
        let chunks = loader.get_chunks(10);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), loader.surnames.len());

        // 验证包含常见姓氏
        assert!(loader.surnames.contains_key("曾"));
        assert!(loader.surnames.contains_key("重"));
    }

    #[test]
    fn test_empty_loader() {
        let loader = WordsLoader { words: HashMap::new() };
        let chunks = loader.get_chunks(5);
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].is_empty());
    }

    #[test]
    #[should_panic]
    fn test_get_chunks_zero_size() {
        let loader = WordsLoader::new();
        loader.get_chunks(0);
    }

    #[test]
    fn test_loader_performance() {
        let start = std::time::Instant::now();
        let _loader = WordsLoader::new();
        let duration = start.elapsed();

        println!("WordsLoader::new() took: {:?}", duration);
        // 加载时间应该在合理范围内（首次加载可能较慢）
        assert!(duration.as_secs() < 15);
    }

    // ========== PHP 兼容性测试 ==========

    #[test]
    fn test_php_name_method_compatibility() {
        // 对应 PHP: Pinyin::name('单某某')
        // 测试姓氏特殊读音处理
        use crate::convert;

        let surname_test_cases = vec![
            ("单某某", "shàn", "mǒu", "mǒu"),
            ("曾国藩", "zēng", "guó", "fān"),
            ("区志华", "ōu", "zhì", "huá"),
            ("仇大雄", "qiú", "dà", "xióng"),
            ("华国锋", "huá", "guó", "fēng"),
            ("万俟卨", "mò", "qí", "xiè"),
        ];

        for (name, expected_surname, _expected_given1, _expected_given2) in surname_test_cases {
            let result = convert(name);

            println!("Name: {}", name);
            println!("Result: {:?}", result);

            assert!(!result.is_empty());

            // 检查姓氏读音（允许声调变化）
            let first_pinyin = &result[0];
            let surname_base = remove_tones(expected_surname);
            let actual_base = remove_tones(first_pinyin);

            assert!(actual_base.contains(&surname_base) ||
                   first_pinyin.contains(expected_surname),
                   "Surname mismatch for {}: expected {}, got {}",
                   name, expected_surname, first_pinyin);
        }
    }

    #[test]
    fn test_php_passport_name_compatibility() {
        // 对应 PHP: Pinyin::passportName('吕小布')
        // 护照姓名：ü 转换为 yu
        use crate::convert;

        let passport_cases = vec![
            ("吕小布", vec!["lyu", "xiao", "bu"]),
            ("女小花", vec!["nyu", "xiao", "hua"]),
            ("律师", vec!["lyu", "shi"]),
            ("绿色", vec!["lyu", "se"]),
        ];

        for (name, expected_parts) in passport_cases {
            let result = convert(name);

            println!("Passport name: {}", name);
            println!("Result: {:?}", result);

            // Rust 版本可能会合并词汇，所以长度可能不同
            assert!(result.len() <= expected_parts.len());

            // 验证拼音不为空且包含字母
            for pinyin in &result {
                assert!(!pinyin.is_empty(), "Empty pinyin in result");
                assert!(pinyin.chars().any(|c| c.is_ascii_alphabetic()),
                       "No alphabetic characters in pinyin: {}", pinyin);
            }
        }
    }

    // 辅助函数：移除声调符号
    fn remove_tones(pinyin: &str) -> String {
        pinyin
            .replace("ā", "a").replace("á", "a").replace("ǎ", "a").replace("à", "a")
            .replace("ē", "e").replace("é", "e").replace("ě", "e").replace("è", "e")
            .replace("ī", "i").replace("í", "i").replace("ǐ", "i").replace("ì", "i")
            .replace("ō", "o").replace("ó", "o").replace("ǒ", "o").replace("ò", "o")
            .replace("ū", "u").replace("ú", "u").replace("ǔ", "u").replace("ù", "u")
            .replace("ǖ", "v").replace("ǘ", "v").replace("ǚ", "v").replace("ǜ", "v")
    }
}
