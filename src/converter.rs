use std::fmt::Display;
use std::str::FromStr;
use crate::error::PingyinError;
use crate::{format_tone, match_word_pinyin, Pinyin, PinyinWord, ToneStyle, YuFormat};

pub struct Converter {
    pub input: String,
    tone_style: ToneStyle,
    yu_format: YuFormat,
}

impl Converter {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            tone_style: ToneStyle::None,
            yu_format: YuFormat::Yu,
        }
    }

    fn convert(&self) -> Vec<PinyinWord> {
        let input_len = self.input.chars().count();
        // todo: 支持选择 Matcher
        let matched_words = match_word_pinyin(&self.input);
        let input_chars: Vec<char> = self.input.chars().collect();

        let mut result = Vec::new();
        let mut i = 0;

        while i < input_len {
            let mut found = false;
            for (word, pinyin) in matched_words.iter() {
                let word_len = word.chars().count();
                if i + word_len <= input_len
                    && &input_chars[i..i + word_len] == word.chars().collect::<Vec<_>>().as_slice()
                {
                    result.push(PinyinWord::from_str(&format!("{}:{}", word, pinyin)).unwrap());
                    i += word_len;
                    found = true;
                    break;
                }
            }

            if !found {
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
            if!result.is_empty() {
                result.push_str(s);
            }
            result.push_str(&word.pinyin.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(s));
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

    fn flatten(&self) -> Vec<PinyinWord> {
        unimplemented!()
    }

    fn as_surnames(&self) -> Vec<PinyinWord> {
        unimplemented!()
    }

    fn only_hans(&self) -> Vec<PinyinWord> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use crate::converter::Converter;
    use crate::ToneStyle;

    #[test]
    fn test_converter() {
        let mut converter = Converter::new("重好");
        assert_eq!(converter.convert().len(), 2);
        assert_eq!(converter.to_string(), "zhòng hǎo");
        assert_eq!(converter.to_permalink(), "zhòng-hǎo");

        converter.without_tone();
        assert_eq!(converter.to_string(), "zhong hao");
        assert_eq!(converter.to_permalink(), "zhong-hao");

        converter.with_tone_style(ToneStyle::Number);
        assert_eq!(converter.to_string(), "zhong4 hao3");
        assert_eq!(converter.to_permalink(), "zhong4-hao3");

        converter.yu_to_u();
        assert_eq!(converter.to_string(), "zhong4 huo3");
        assert_eq!(converter.to_permalink(), "zhong4-huo3");

        converter.yu_to_v();
        assert_eq!(converter.to_string(), "zhong4 hvo3");
        assert_eq!(converter.to_permalink(), "zhong4-hvo3");
    }
}