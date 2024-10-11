use crate::matcher::{match_surname_pinyin, match_word_pinyin};
use crate::{pinyin, Pinyin, PinyinWord, ToneStyle, YuFormat};
use std::pin;
use std::str::FromStr;

pub struct Converter {
    pub input: String,
    tone_style: ToneStyle,
    yu_format: YuFormat,
    surname_mode: bool,
    flatten: bool,
    only_hans: bool,
}

impl Converter {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            tone_style: ToneStyle::None,
            yu_format: YuFormat::Yu,
            surname_mode: false,
            flatten: false,
            only_hans: false,
        }
    }

    fn convert(&self) -> Vec<PinyinWord> {
        let input_len = self.input.chars().count();
        let matched_words = if self.surname_mode {
            match_surname_pinyin(&self.input)
        } else {
            match_word_pinyin(&self.input)
        };

        #[cfg(test)]
        println!("matched_words: {:?}", matched_words);

        let input_chars: Vec<char> = self.input.chars().collect();

        let mut result = Vec::new();
        let mut i = 0;
        let mut skip_matched_len = 0;

        if self.surname_mode {
            // 提出姓氏的拼音，第一个匹配的就是姓氏，把它放到结果里
            if let Some((word, pinyin)) = matched_words.first() {
                let pinyin_word = PinyinWord::from_str(&format!("{}:{}", word, pinyin)).unwrap();
                result.push(pinyin_word);
                i += word.chars().count();
                skip_matched_len = 1;
            }
        }

        while i < input_len {
            let mut found = false;
            for (word, pinyin) in matched_words.iter().skip(skip_matched_len) {
                let word_len = word.chars().count();
                if i + word_len <= input_len
                    && &input_chars[i..i + word_len] == word.chars().collect::<Vec<_>>().as_slice()
                {
                    let mut pinyin_word =
                        PinyinWord::from_str(&format!("{}:{}", word, pinyin)).unwrap();

                    // 单子多音字，只取第一个音
                    if self.flatten && pinyin_word.word.len() == 1 {
                        pinyin_word.pinyin.truncate(1);
                    }

                    result.push(pinyin_word);
                    i += word_len;
                    found = true;
                    break;
                }
            }

            if !found && !self.only_hans {
                result.push(PinyinWord::new(
                    input_chars[i].to_string(),
                    vec![Pinyin::new(&input_chars[i].to_string(), 5).into()],
                ));
                i += 1;
            }
        }

        result
    }

    fn to_string(&self) -> String {
        self.to_string_with(" ")
    }

    fn to_permalink(&self) -> String {
        self.to_string_with("-")
    }

    fn to_string_with(&self, s: &str) -> String {
        let mut result = String::new();

        for word in self.convert() {
            if !result.is_empty() {
                result.push_str(s);
            }

            result.push_str(
                &word
                    .pinyin
                    .iter()
                    .map(|p| p.format(self.tone_style))
                    .collect::<Vec<_>>()
                    .join(s),
            );
        }

        result.trim_end().to_string()
    }

    fn with_tone_style(&mut self, style: ToneStyle) -> &mut Self {
        self.tone_style = style;
        self
    }

    fn without_tone(&mut self) -> &mut Self {
        self.tone_style = ToneStyle::None;
        self
    }

    fn yu_to_yu(&mut self) -> &mut Self {
        self.yu_format = YuFormat::Yu;
        self
    }

    fn yu_to_u(&mut self) -> &mut Self {
        self.yu_format = YuFormat::U;
        self
    }

    fn yu_to_v(&mut self) -> &mut Self {
        self.yu_format = YuFormat::V;
        self
    }

    fn flatten(&mut self) -> &mut Self {
        self.flatten = true;
        self
    }

    fn as_surnames(&mut self) -> &mut Self {
        self.surname_mode = true;
        self.flatten();
        self
    }

    fn only_hans(&mut self) -> &mut Self {
        self.only_hans = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::Converter;
    use crate::ToneStyle;

    #[test]
    fn test_convert() {
        let mut converter = Converter::new("重好");
        assert_eq!(converter.convert().len(), 2); // ["重": ["zhòng", "chóng", "tóng"], "好": ["hǎo", "hào"]]
        assert_eq!(converter.to_string(), "zhòng chóng tóng hǎo hào");

        converter.flatten();
        assert_eq!(converter.to_string(), "zhòng hǎo");
        assert_eq!(converter.to_permalink(), "zhòng-hǎo");
    }

    #[test]
    fn test_convert_with_default_tone_style() {
        let mut converter = Converter::new("重好");
        assert_eq!(
            converter.with_tone_style(ToneStyle::None).convert().len(),
            2
        );
        assert_eq!(converter.to_string(), "zhong4 hao3");
        assert_eq!(converter.to_permalink(), "zhong4-hao3");
    }

    #[test]
    fn test_convert_with_number_tone_style() {
        let mut converter = Converter::new("重好");
        assert_eq!(
            converter.with_tone_style(ToneStyle::Number).convert().len(),
            2
        );
        assert_eq!(converter.to_string(), "zhong4 chong2 tong2 hao3 hao4");
        assert_eq!(converter.to_permalink(), "zhong4-chong2-tong2-hao3-hao4");
    }

    #[test]
    fn test_convert_with_mark_tone_style() {
        let mut converter = Converter::new("重好");
        assert_eq!(
            converter.with_tone_style(ToneStyle::Mark).convert().len(),
            2
        );
        assert_eq!(converter.to_string(), "zhòng chóng tóng hǎo hào");
        assert_eq!(converter.to_permalink(), "zhòng-chóng-tóng-hǎo-hào");
    }

    #[test]
    fn test_convert_without_tone() {
        let mut converter = Converter::new("重好");
        assert_eq!(converter.without_tone().convert().len(), 2);
        assert_eq!(converter.to_string(), "zhong hao");
        assert_eq!(converter.to_permalink(), "zhong-hao");
    }

    #[test]
    fn test_convert_yu() {
        // lv/lu/lyu
        let mut converter = Converter::new("旅行");
        assert_eq!(converter.convert().len(), 1);
        assert_eq!(converter.to_string(), "lǚ xíng");

        let mut converter = Converter::new("旅行");
        converter.yu_to_u();
        assert_eq!(converter.to_string(), "lǚ xíng");

        let mut converter = Converter::new("旅行");
        converter.yu_to_v();
        assert_eq!(converter.to_string(), "lv xing");

        // without tone same as yu_to_v
        let mut converter = Converter::new("旅行");
        converter.without_tone();
        assert_eq!(converter.to_string(), "lv xing");
    }

    #[test]
    fn test_convert_as_surnames() {
        let mut converter = Converter::new("单单单");
        converter.as_surnames();
        assert_eq!(converter.convert().len(), 2);
        assert_eq!(converter.to_string(), "shàn dān dān");
    }
}
