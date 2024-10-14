mod converter;
mod error;
mod loader;
mod matcher;
mod pinyin;

use std::{fmt::Display, str::FromStr};

use crate::error::PingyinError;

#[derive(Debug)]
pub struct Pinyin {
    pub pinyin: String,
    pub tone: u8,
}

impl Pinyin {
    #[allow(dead_code)]
    pub fn new(pinyin: &str, tone: u8) -> Self {
        assert!((1..=5).contains(&tone));

        Self {
            pinyin: pinyin.to_string(),
            tone,
        }
    }

    #[allow(dead_code)]
    pub fn is_toneless(&self) -> bool {
        self.tone == 5
    }

    #[allow(dead_code)]
    pub fn format(&self, style: ToneStyle) -> String {
        match style {
            ToneStyle::Number => self.to_string(),
            ToneStyle::Mark => format_to_mark(&self.pinyin, self.tone),
            ToneStyle::None => format_to_no_tone(&self.pinyin),
        }
    }

    pub fn format_with_yu(&self, yu_format: YuFormat) -> String {
        match yu_format {
            YuFormat::Yu => self.pinyin.clone(),
            YuFormat::U => self.pinyin.replace("ü", "u"),
            YuFormat::V => self.pinyin.replace("ü", "v"),
        }
    }
}

impl Display for Pinyin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.pinyin, self.tone)
    }
}

impl PartialEq for Pinyin {
    fn eq(&self, other: &Self) -> bool {
        self.pinyin == other.pinyin && self.tone == other.tone
    }
}

impl FromStr for Pinyin {
    type Err = PingyinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tone = 5;
        let mut end = s.len();
        let c = match s.chars().last() {
            Some(c) => c,
            None => {
                return Err(PingyinError::ParseStrError(s.to_string()));
            }
        };
        if c.is_numeric() {
            tone = c.to_digit(10).unwrap() as u8;
            end -= 1;
        }

        let pinyin: String = s.chars().take(end).collect();
        Ok(Self { pinyin, tone })
    }
}

#[derive(Debug)]
pub struct PinyinWord {
    // "重庆"
    pub word: String,
    // [["chong", 2], ["qing", 4]]
    pub pinyin: Vec<Pinyin>,
}

impl PinyinWord {
    #[allow(dead_code)]
    pub fn new(word: String, pinyin: Vec<Pinyin>) -> Self {
        Self { word, pinyin }
    }
}

impl Display for PinyinWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pinyin = self
            .pinyin
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{}:{}", self.word, pinyin)
    }
}

impl PartialEq for PinyinWord {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word && self.pinyin == other.pinyin
    }
}

impl FromStr for PinyinWord {
    type Err = PingyinError;

    // "重:zhong4 chong2" -> PinyinWord { word: "重", pinyin: [["zhong", 4], ["chong", 2]] }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let word = parts
            .next()
            .ok_or_else(|| PingyinError::ParseStrError(s.to_string()))?
            .to_string();
        let mut pinyin = vec![];
        for p in parts
            .next()
            .ok_or_else(|| PingyinError::ParseStrError(s.to_string()))?
            .split(' ')
        {
            pinyin.push(Pinyin::from_str(p)?);
        }

        Ok(Self { word, pinyin })
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ToneStyle {
    Number,
    Mark,
    None,
}

impl FromStr for ToneStyle {
    type Err = PingyinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "number" => Ok(Self::Number),
            "mark" => Ok(Self::Mark),
            "none" => Ok(Self::None),
            _ => Err(PingyinError::ParseStrError(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum YuFormat {
    Yu,
    U,
    V,
}

pub fn format_to_mark(pinyin: &str, tone: u8) -> String {
    // find the vowel to mark
    // if the vowel is 'i' or 'u' or 'ü', find the next vowel
    let mut chars: Vec<char> = pinyin.chars().collect();
    let mut last_vowel_idx: i8 = -1;

    for (idx, c) in chars.iter().enumerate() {
        if "aeiouü".contains(*c) {
            last_vowel_idx = idx as i8;
            if *c != 'i' || *c != 'u' || *c != 'ü' {
                break;
            }
        }
    }

    if last_vowel_idx > -1 {
        let vowel = chars[last_vowel_idx as usize];
        chars[last_vowel_idx as usize] = mark_vowel(vowel, tone);
    }

    chars.into_iter().collect()
}

pub fn format_to_no_tone(pinyin: &str) -> String {
    pinyin.chars().map(|c| unmark_vowel(c)).collect::<String>()
}

pub fn unmark_vowel(vowel: char) -> char {
    match vowel {
        'ā' | 'á' | 'ǎ' | 'à' => 'a',
        'ē' | 'é' | 'ě' | 'è' => 'e',
        'ī' | 'í' | 'ǐ' | 'ì' => 'i',
        'ō' | 'ó' | 'ǒ' | 'ò' => 'o',
        'ū' | 'ú' | 'ǔ' | 'ù' => 'u',
        'ǖ' | 'ǘ' | 'ǚ' | 'ǜ' => 'ü',
        _ => vowel,
    }
}

pub fn mark_vowel(vowel: char, tone: u8) -> char {
    if tone == 0 || tone == 5 {
        return vowel;
    }

    let tone_marks = [
        'ā', 'á', 'ǎ', 'à', 'ē', 'é', 'ě', 'è', 'ī', 'í', 'ǐ', 'ì', 'ō', 'ó', 'ǒ', 'ò', 'ū', 'ú',
        'ǔ', 'ù', 'ǖ', 'ǘ', 'ǚ', 'ǜ',
    ];
    let index = match vowel {
        'a' => tone,
        'e' => tone + 4,
        'i' => tone + 8,
        'o' => tone + 12,
        'u' => tone + 16,
        'ü' => tone + 20,
        _ => panic!("Invalid vowel {}", vowel),
    } as usize;

    tone_marks[index - 1]
}

// zhōng -> zhong1
pub fn transform_mark_to_number(pinyin: &str) -> Pinyin {
    let mut chars: Vec<char> = pinyin.chars().collect();
    let mut tone = 5;

    let unmarked_vowels = ['a', 'e', 'i', 'o', 'u', 'ü'];
    let tone_mapping = [("āēīōūǖ", 1), ("áéíóúǘ", 2), ("ǎěǐǒǔǚ", 3), ("àèìòùǜ", 4)];

    for (marked_vowels, t) in tone_mapping {
        if let Some((idx, marked_vowel)) = chars
            .iter()
            .enumerate()
            .find(|(_, c)| marked_vowels.contains(**c))
        {
            let vowel_index = marked_vowels
                .chars()
                .position(|c| c == *marked_vowel)
                .unwrap();
            let vowel = unmarked_vowels[vowel_index];
            tone = t;
            chars[idx] = vowel;
            break;
        }
    }

    Pinyin::new(&chars.into_iter().collect::<String>(), tone)
}

#[cfg(test)]
mod tests {
    use crate::{mark_vowel, transform_mark_to_number, Pinyin, PinyinWord, ToneStyle, YuFormat};
    use std::str::FromStr;

    #[test]
    fn test_pinyin_new() {
        let pinyin = Pinyin::new("zhong", 4);
        assert_eq!(pinyin.pinyin, "zhong");
        assert_eq!(pinyin.tone, 4);

        let pinyin = Pinyin::new("a", 5);
        assert_eq!(pinyin.pinyin, "a");
    }

    #[test]
    #[should_panic]
    fn test_pinyin_new_panic_with_invalid_tone() {
        let _pinyin = Pinyin::new("zhong", 6);
    }

    #[test]
    #[should_panic]
    fn test_pinyin_new_panic_with_zero_tone() {
        let _pinyin = Pinyin::new("zhong", 0);
    }

    #[test]
    fn test_pinyin_is_toneless() {
        let pinyin = Pinyin::new("zhong", 4);
        assert!(!pinyin.is_toneless());

        let pinyin = Pinyin::new("zhong", 5);
        assert!(pinyin.is_toneless());
    }

    #[test]
    fn test_pinyin_to_string() {
        let pinyin = Pinyin::new("zhong", 4);
        assert_eq!(pinyin.to_string(), "zhong4");
    }

    #[test]
    fn test_pinyin_format() {
        let pinyin = Pinyin::new("zhong", 4);
        assert_eq!(pinyin.format(ToneStyle::Number), "zhong4");
        assert_eq!(pinyin.format(ToneStyle::Mark), "zhòng");
        assert_eq!(pinyin.format(ToneStyle::None), "zhong");

        let pinyin = Pinyin::new("a", 5);
        assert_eq!(pinyin.format(ToneStyle::Number), "a5");
        assert_eq!(pinyin.format(ToneStyle::Mark), "a");
        assert_eq!(pinyin.format(ToneStyle::None), "a");

        let pinyin = Pinyin::from_str("rén").unwrap();
        assert_eq!(pinyin.format(ToneStyle::None), "ren");
    }

    #[test]
    fn test_pinyin_format_with_yu() {
        let pinyin = Pinyin::new("lǚ xíng", 4);
        assert_eq!(pinyin.format(ToneStyle::Mark), "lǚ xíng");
        assert_eq!(pinyin.format(ToneStyle::None), "lv xing");
        assert_eq!(pinyin.format_with_yu(YuFormat::Yu), "lyu xíng");
        assert_eq!(pinyin.format_with_yu(YuFormat::U), "lu xíng");
        assert_eq!(pinyin.format_with_yu(YuFormat::V), "lv xíng");
    }

    #[test]
    fn test_pinyin_from_string() {
        let pinyin = Pinyin::from_str("zhong4").unwrap();
        assert_eq!(pinyin.pinyin, "zhong");
        assert_eq!(pinyin.tone, 4);

        let pinyin = Pinyin::from_str("zhong").unwrap();
        assert_eq!(pinyin.pinyin, "zhong");
        assert_eq!(pinyin.tone, 5);
    }

    #[test]
    fn test_pinyin_word_new() {
        let pinyin = vec![Pinyin::new("zhong", 4), Pinyin::new("chong", 2)];
        let pinyin_word = PinyinWord::new("重".to_string(), pinyin);
        assert_eq!(pinyin_word.word, "重");
        assert_eq!(pinyin_word.pinyin.len(), 2);
    }

    #[test]
    fn test_pinyin_word_to_string() {
        let pinyin = vec![Pinyin::new("zhong", 4), Pinyin::new("chong", 2)];
        let pinyin_word = PinyinWord::new("重".to_string(), pinyin);
        assert_eq!(pinyin_word.to_string(), "重:zhong4 chong2");
    }

    #[test]
    fn test_pinyin_word_from_string() {
        let pinyin_word = PinyinWord::from_str("重:zhong4 chong2").unwrap();
        assert_eq!(pinyin_word.word, "重");
        assert_eq!(pinyin_word.pinyin.len(), 2);
        assert_eq!(pinyin_word.to_string(), "重:zhong4 chong2");

        let pinyin_word = PinyinWord::from_str("重庆:chong2 qing4").unwrap();
        assert_eq!(pinyin_word.word, "重庆");
        assert_eq!(pinyin_word.pinyin.len(), 2);
        assert_eq!(pinyin_word.to_string(), "重庆:chong2 qing4");

        let pinyin_word = PinyinWord::from_str("重庆口味:chong2 qing4 kou3 wei4").unwrap();
        assert_eq!(pinyin_word.word, "重庆口味");
        assert_eq!(pinyin_word.pinyin.len(), 4);
        assert_eq!(pinyin_word.to_string(), "重庆口味:chong2 qing4 kou3 wei4");
    }

    #[test]
    fn test_mark_vowel() {
        assert_eq!(mark_vowel('a', 1), 'ā');
        assert_eq!(mark_vowel('a', 2), 'á');
        assert_eq!(mark_vowel('a', 3), 'ǎ');
        assert_eq!(mark_vowel('a', 4), 'à');
        assert_eq!(mark_vowel('a', 5), 'a');
        assert_eq!(mark_vowel('e', 1), 'ē');
        assert_eq!(mark_vowel('e', 2), 'é');
        assert_eq!(mark_vowel('e', 3), 'ě');
        assert_eq!(mark_vowel('e', 4), 'è');
        assert_eq!(mark_vowel('e', 5), 'e');
        assert_eq!(mark_vowel('i', 1), 'ī');
        assert_eq!(mark_vowel('i', 2), 'í');
        assert_eq!(mark_vowel('i', 3), 'ǐ');
        assert_eq!(mark_vowel('i', 4), 'ì');
        assert_eq!(mark_vowel('i', 5), 'i');
        assert_eq!(mark_vowel('o', 1), 'ō');
        assert_eq!(mark_vowel('o', 2), 'ó');
        assert_eq!(mark_vowel('o', 3), 'ǒ');
        assert_eq!(mark_vowel('o', 4), 'ò');
        assert_eq!(mark_vowel('o', 5), 'o');
        assert_eq!(mark_vowel('u', 1), 'ū');
        assert_eq!(mark_vowel('u', 2), 'ú');
        assert_eq!(mark_vowel('u', 3), 'ǔ');
        assert_eq!(mark_vowel('u', 4), 'ù');
        assert_eq!(mark_vowel('u', 5), 'u');
        assert_eq!(mark_vowel('ü', 1), 'ǖ');
        assert_eq!(mark_vowel('ü', 2), 'ǘ');
        assert_eq!(mark_vowel('ü', 3), 'ǚ');
        assert_eq!(mark_vowel('ü', 4), 'ǜ');
        assert_eq!(mark_vowel('ü', 5), 'ü');
    }

    #[test]
    #[should_panic]
    fn test_mark_vowel_panic_with_invalid_vowel() {
        mark_vowel('b', 1);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn test_mark_vowel_panic_with_invalid_tone() {
        mark_vowel('a', 6);
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn test_mark_vowel_panic_with_zero_tone() {
        mark_vowel('a', 0);
    }

    #[test]
    fn test_mark_vowel_with_toneless() {
        assert_eq!(mark_vowel('a', 5), 'a');
    }

    #[test]
    fn test_transform_mark_to_number() {
        assert_eq!(transform_mark_to_number("zhōng"), Pinyin::new("zhong", 1));
        assert_eq!(transform_mark_to_number("zhóng"), Pinyin::new("zhong", 2));
        assert_eq!(transform_mark_to_number("zhǒng"), Pinyin::new("zhong", 3));
        assert_eq!(transform_mark_to_number("zhòng"), Pinyin::new("zhong", 4));
        assert_eq!(transform_mark_to_number("zhong"), Pinyin::new("zhong", 5));
        assert_eq!(transform_mark_to_number("en"), Pinyin::new("en", 5));
    }
}
