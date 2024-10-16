use crate::matcher::{match_surname_pinyin, match_word_pinyin};
use crate::{Pinyin, PinyinWord, ToneStyle, YuFormat};
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
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            tone_style: ToneStyle::Mark,
            yu_format: YuFormat::U,
            surname_mode: false,
            flatten: false,
            only_hans: false,
        }
    }

    pub fn convert(&self) -> Vec<PinyinWord> {
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

                    // 多音字，只取第一个音
                    if self.flatten {
                        pinyin_word
                            .pinyin
                            .truncate(word_len);
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

        #[cfg(test)]
        println!("match result: {:#?}", result);

        result
    }

    pub fn to_string(&self) -> String {
        self.to_string_with(" ")
    }

    pub fn to_permalink(&self) -> String {
        self.to_string_with("-")
    }

    pub fn to_string_with(&self, s: &str) -> String {
        let mut result = String::new();

        for word in self.convert() {
            if !result.is_empty() {
                result.push_str(s);
            }

            result.push_str(
                &word
                    .pinyin
                    .iter()
                    .map(|p| p.format_with_yu(self.tone_style, self.yu_format))
                    .collect::<Vec<_>>()
                    .join(s),
            );
        }

        result.trim_end().to_string()
    }

    pub fn with_tone_style(&mut self, style: ToneStyle) -> &mut Self {
        self.tone_style = style;
        self
    }

    pub fn without_tone(&mut self) -> &mut Self {
        self.tone_style = ToneStyle::None;
        self
    }

    pub fn yu_to_yu(&mut self) -> &mut Self {
        self.yu_format = YuFormat::Yu;
        self
    }

    pub fn yu_to_u(&mut self) -> &mut Self {
        self.yu_format = YuFormat::U;
        self
    }

    pub fn yu_to_v(&mut self) -> &mut Self {
        self.yu_format = YuFormat::V;
        self.tone_style = ToneStyle::None;
        self
    }

    pub fn flatten(&mut self) -> &mut Self {
        self.flatten = true;
        self
    }

    pub fn as_surnames(&mut self) -> &mut Self {
        self.surname_mode = true;
        self.flatten();
        self
    }

    pub fn only_hans(&mut self) -> &mut Self {
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
        println!("{:#?}", converter.convert());
        assert_eq!(converter.convert().len(), 2); // ["重": ["zhòng", "chóng", "tóng"], "好": ["hǎo", "hào"]]
        assert_eq!(converter.to_string(), "zhòng chóng tóng hǎo hào");

        converter.flatten();
        println!("{:?}", converter.convert());
        assert_eq!(converter.to_string(), "zhòng hǎo");
    }

    #[test]
    fn test_convert_with_default_tone_style() {
        let mut converter = Converter::new("你人");
        assert_eq!(
            converter.with_tone_style(ToneStyle::None).convert().len(),
            2
        );
        println!("{:?}", converter.to_string());
        assert_eq!(converter.to_string(), "ni ren")
    }

    #[test]
    fn test_convert_with_number_tone_style() {
        let mut converter = Converter::new("你人");
        assert_eq!(
            converter.with_tone_style(ToneStyle::Number).convert().len(),
            2
        );
        assert_eq!(converter.to_string(), "ni3 ren2");
    }

    #[test]
    fn test_convert_with_mark_tone_style() {
        let mut converter = Converter::new("你人");
        assert_eq!(
            converter.with_tone_style(ToneStyle::Mark).convert().len(),
            2
        );
        assert_eq!(converter.to_string(), "nǐ rén")
    }

    #[test]
    fn test_convert_without_tone() {
        let mut converter = Converter::new("你人");
        assert_eq!(converter.without_tone().convert().len(), 2);
        assert_eq!(converter.to_string(), "ni ren");
    }

    #[test]
    fn test_convert_yu() {
        // lv/lu/lyu
        let converter = Converter::new("旅行");
        assert_eq!(converter.convert().len(), 1);
        assert_eq!(converter.to_string(), "lǚ xíng");

        let mut converter = Converter::new("旅行");
        converter.yu_to_u();
        assert_eq!(converter.to_string(), "lǚ xíng");

        let mut converter = Converter::new("旅行");
        converter.yu_to_v();
        println!("{:?}", converter.convert());
        assert_eq!(converter.to_string(), "lv xing");

        // without tone same as yu_to_v
        let mut converter = Converter::new("旅行");
        converter.without_tone();
        assert_eq!(converter.to_string(), "lv xing");

        // 护照模式需要注意：
        // // lue/nue
        // let pinyin = Pinyin::new("lüe", 4);
        // assert_eq!(pinyin.format_with_yu(ToneStyle::Mark, YuFormat::Yu), "lue");

        // let pinyin = Pinyin::new("nüe", 4);
        // assert_eq!(pinyin.format_with_yu(ToneStyle::Mark, YuFormat::Yu), "nue");
    }

    #[test]
    fn test_convert_as_surnames() {
        let mut converter = Converter::new("单单单");
        converter.as_surnames();
        println!("{:?}", converter.convert());
        assert_eq!(converter.convert().len(), 2);
        assert_eq!(converter.to_string(), "shàn dān dān");
    }
}
