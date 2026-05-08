mod error;
mod loader;
mod matcher;
mod pinyin;

use std::fmt;
use std::ops::Index;
use std::sync::OnceLock;

pub use error::PinyinError;
use error::Result;
use loader::Lexicon;
use matcher::{Matcher, Segment, group_unmatched_for_sentence};
use pinyin::{first_pronunciation, format_phrase, initials_token, slug_token, split_phrase};

static LEXICON: OnceLock<Lexicon> = OnceLock::new();
static DEFAULT_MATCHER: OnceLock<Matcher> = OnceLock::new();
static PLAIN_MATCHER: OnceLock<Matcher> = OnceLock::new();
static SURNAME_MATCHER: OnceLock<Matcher> = OnceLock::new();

const VALID_DELIMITERS: [&str; 4] = ["-", "_", ".", ""];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToneStyle {
    #[default]
    Mark,
    Number,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YuStyle {
    #[default]
    Umlaut,
    V,
    Yu,
    U,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinyinWord {
    pub text: String,
    pub pinyin: String,
}

impl PinyinWord {
    pub fn new(text: impl Into<String>, pinyin: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            pinyin: pinyin.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinyinResult {
    words: Vec<PinyinWord>,
    tone_style: ToneStyle,
    yu_style: YuStyle,
}

impl PinyinResult {
    pub fn new(words: Vec<PinyinWord>) -> Self {
        Self {
            words,
            tone_style: ToneStyle::Mark,
            yu_style: YuStyle::Umlaut,
        }
    }

    pub fn with_tone_style(mut self, style: ToneStyle) -> Self {
        self.tone_style = style;
        self
    }

    pub fn without_tone(mut self) -> Self {
        self.tone_style = ToneStyle::None;
        if self.yu_style == YuStyle::Umlaut {
            self.yu_style = YuStyle::V;
        }
        self
    }

    pub fn flatten(self) -> Self {
        self
    }

    pub fn yu_to_v(mut self) -> Self {
        self.yu_style = YuStyle::V;
        self
    }

    pub fn yu_to_yu(mut self) -> Self {
        self.yu_style = YuStyle::Yu;
        self
    }

    pub fn yu_to_u(mut self) -> Self {
        self.yu_style = YuStyle::U;
        self
    }

    pub fn yu_to_umlaut(mut self) -> Self {
        self.yu_style = YuStyle::Umlaut;
        self
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    pub fn words(&self) -> &[PinyinWord] {
        &self.words
    }

    pub fn iter(&self) -> impl Iterator<Item = String> + '_ {
        self.words
            .iter()
            .map(|word| format_phrase(&word.pinyin, self.tone_style, self.yu_style))
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.iter().collect()
    }

    pub fn join(&self, separator: &str) -> String {
        self.to_string_with(separator)
    }

    pub fn to_string_with(&self, separator: &str) -> String {
        self.iter().collect::<Vec<_>>().join(separator)
    }

    pub fn to_permalink(&self) -> String {
        self.clone().without_tone().yu_to_v().to_string_with("-")
    }
}

impl fmt::Display for PinyinResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_string_with(" "))
    }
}

impl Index<usize> for PinyinResult {
    type Output = PinyinWord;

    fn index(&self, index: usize) -> &Self::Output {
        &self.words[index]
    }
}

impl IntoIterator for PinyinResult {
    type IntoIter = std::vec::IntoIter<PinyinWord>;
    type Item = PinyinWord;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct PinyinConfig {
    pub enable_polyphone: bool,
    pub prefer_long_words: bool,
    pub max_input_length: usize,
}

impl Default for PinyinConfig {
    fn default() -> Self {
        Self {
            enable_polyphone: false,
            prefer_long_words: true,
            max_input_length: 10_000,
        }
    }
}

impl PinyinConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_polyphone(mut self, enabled: bool) -> Self {
        self.enable_polyphone = enabled;
        self
    }

    pub fn with_long_words(mut self, enabled: bool) -> Self {
        self.prefer_long_words = enabled;
        self
    }

    pub fn with_max_length(mut self, length: usize) -> Self {
        self.max_input_length = length;
        self
    }

    pub fn validate(&self) -> Result<()> {
        if self.max_input_length == 0 {
            return Err(PinyinError::InvalidMaxInputLength(self.max_input_length));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Converter {
    input: String,
    tone_style: ToneStyle,
    yu_style: YuStyle,
    surname_mode: bool,
    only_hans: bool,
    keep_punctuation: bool,
    split_words: bool,
}

impl Converter {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            tone_style: ToneStyle::Mark,
            yu_style: YuStyle::Umlaut,
            surname_mode: false,
            only_hans: false,
            keep_punctuation: true,
            split_words: true,
        }
    }

    pub fn with_tone_style(mut self, style: ToneStyle) -> Self {
        self.tone_style = style;
        self
    }

    pub fn without_tone(mut self) -> Self {
        self.tone_style = ToneStyle::None;
        if self.yu_style == YuStyle::Umlaut {
            self.yu_style = YuStyle::V;
        }
        self
    }

    pub fn yu_to_v(mut self) -> Self {
        self.yu_style = YuStyle::V;
        self
    }

    pub fn yu_to_yu(mut self) -> Self {
        self.yu_style = YuStyle::Yu;
        self
    }

    pub fn yu_to_u(mut self) -> Self {
        self.yu_style = YuStyle::U;
        self
    }

    pub fn yu_to_umlaut(mut self) -> Self {
        self.yu_style = YuStyle::Umlaut;
        self
    }

    pub fn flatten(self) -> Self {
        self
    }

    pub fn as_surnames(mut self) -> Self {
        self.surname_mode = true;
        self
    }

    pub fn only_hans(mut self) -> Self {
        self.only_hans = true;
        self
    }

    pub fn no_punctuation(mut self) -> Self {
        self.keep_punctuation = false;
        self
    }

    pub fn raw_words(mut self) -> Self {
        self.split_words = false;
        self
    }

    pub fn convert(&self) -> PinyinResult {
        let segments = if self.surname_mode {
            name_segments(&self.input)
        } else {
            default_matcher().segments(&self.input)
        };

        let words = result_words(
            group_unmatched_for_sentence(segments),
            self.only_hans,
            self.keep_punctuation,
            self.split_words,
        );

        PinyinResult::new(words)
            .with_tone_style(self.tone_style)
            .with_yu_style(self.yu_style)
    }

    pub fn to_string_with(&self, separator: &str) -> String {
        self.convert().to_string_with(separator)
    }

    pub fn to_permalink(&self) -> String {
        self.clone()
            .without_tone()
            .yu_to_v()
            .no_punctuation()
            .convert()
            .to_permalink()
    }
}

impl fmt::Display for Converter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.convert().to_string())
    }
}

trait WithYuStyle {
    fn with_yu_style(self, style: YuStyle) -> Self;
}

impl WithYuStyle for PinyinResult {
    fn with_yu_style(mut self, style: YuStyle) -> Self {
        self.yu_style = style;
        self
    }
}

pub struct Pinyin;

impl Pinyin {
    pub fn sentence(input: &str) -> PinyinResult {
        Converter::new(input).convert()
    }

    pub fn phrase(input: &str) -> PinyinResult {
        Converter::new(input).no_punctuation().convert()
    }

    pub fn full_sentence(input: &str) -> PinyinResult {
        Self::sentence(input)
    }

    pub fn permalink(input: &str) -> String {
        Self::permalink_with(input, "-").expect("default delimiter is valid")
    }

    pub fn permalink_with(input: &str, delimiter: &str) -> Result<String> {
        if !VALID_DELIMITERS.contains(&delimiter) {
            return Err(PinyinError::invalid_delimiter(delimiter));
        }

        let tokens = Converter::new(input)
            .without_tone()
            .yu_to_v()
            .no_punctuation()
            .convert()
            .iter()
            .map(|token| slug_token(&token))
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>();

        Ok(tokens.join(delimiter))
    }

    pub fn abbr(input: &str) -> PinyinResult {
        let words = Converter::new(input)
            .without_tone()
            .yu_to_v()
            .no_punctuation()
            .convert()
            .iter()
            .filter_map(|token| {
                let initial = initials_token(&token);
                (!initial.is_empty()).then(|| PinyinWord::new(token, initial))
            })
            .collect();

        PinyinResult::new(words).without_tone().yu_to_v()
    }

    pub fn name_abbr(input: &str) -> PinyinResult {
        let words = Self::name(input)
            .without_tone()
            .yu_to_v()
            .iter()
            .filter_map(|token| {
                let initial = initials_token(&token);
                (!initial.is_empty()).then(|| PinyinWord::new(token, initial))
            })
            .collect();

        PinyinResult::new(words).without_tone().yu_to_v()
    }

    pub fn name(input: &str) -> PinyinResult {
        Converter::new(input).as_surnames().convert()
    }

    pub fn passport_name(input: &str) -> PinyinResult {
        Self::name(input).without_tone().yu_to_yu()
    }

    pub fn chars(input: &str) -> PinyinResult {
        let words = input
            .chars()
            .filter_map(|ch| {
                lexicon().char_pinyin(ch).map(|pinyin| {
                    PinyinWord::new(ch.to_string(), first_pronunciation(pinyin).to_string())
                })
            })
            .collect();
        PinyinResult::new(words)
    }

    pub fn heteronym(input: &str) -> Vec<(char, Vec<String>)> {
        input
            .chars()
            .filter_map(|ch| {
                lexicon().heteronyms(ch).map(|items| {
                    (
                        ch,
                        items
                            .into_iter()
                            .map(str::to_string)
                            .collect::<Vec<String>>(),
                    )
                })
            })
            .collect()
    }
}

pub fn match_word_pinyin(input: &str) -> Vec<(String, String)> {
    default_matcher()
        .segments(input)
        .into_iter()
        .filter(|segment| segment.matched)
        .map(|segment| (segment.text, segment.pinyin))
        .collect()
}

pub fn convert(input: &str) -> Vec<String> {
    default_matcher()
        .segments(input)
        .into_iter()
        .map(|segment| segment.pinyin)
        .collect()
}

pub fn convert_as_surname(input: &str) -> Vec<String> {
    surname_matcher()
        .segments(input)
        .into_iter()
        .map(|segment| segment.pinyin)
        .collect()
}

pub fn convert_safe(input: &str) -> Result<Vec<String>> {
    convert_with_config(input, &PinyinConfig::default())
}

pub fn convert_with_config(input: &str, config: &PinyinConfig) -> Result<Vec<String>> {
    config.validate()?;
    if input.len() > config.max_input_length {
        return Err(PinyinError::InputTooLong {
            actual: input.len(),
            max: config.max_input_length,
        });
    }

    let mut result = if config.prefer_long_words {
        convert(input)
    } else {
        input
            .chars()
            .map(|ch| {
                lexicon()
                    .char_pinyin(ch)
                    .map(str::to_string)
                    .unwrap_or_else(|| ch.to_string())
            })
            .collect()
    };

    if !config.enable_polyphone {
        for item in &mut result {
            *item = first_pronunciation(item).to_string();
        }
    }

    Ok(result)
}

fn result_words(
    segments: Vec<Segment>,
    only_hans: bool,
    keep_punctuation: bool,
    split_words: bool,
) -> Vec<PinyinWord> {
    let mut words = Vec::with_capacity(segments.len());

    for segment in segments {
        if only_hans && !segment.matched {
            continue;
        }
        if !keep_punctuation && is_punctuation_token(&segment.text) {
            continue;
        }

        if segment.matched {
            push_matched_words(&mut words, &segment, split_words);
        } else if !segment.text.trim().is_empty() {
            words.push(PinyinWord::new(segment.text, segment.pinyin));
        }
    }

    words
}

fn push_matched_words(words: &mut Vec<PinyinWord>, segment: &Segment, split_words: bool) {
    let char_count = segment.text.chars().count();
    let syllables = split_phrase(&segment.pinyin);

    if !split_words {
        let pinyin = if char_count == 1 {
            first_pronunciation(&segment.pinyin).to_string()
        } else {
            segment.pinyin.clone()
        };
        words.push(PinyinWord::new(segment.text.clone(), pinyin));
        return;
    }

    if char_count == 1 {
        words.push(PinyinWord::new(
            segment.text.clone(),
            first_pronunciation(&segment.pinyin).to_string(),
        ));
        return;
    }

    let chars = segment.text.chars().collect::<Vec<_>>();
    if chars.len() == syllables.len() {
        for (ch, syllable) in chars.into_iter().zip(syllables) {
            words.push(PinyinWord::new(ch.to_string(), syllable.to_string()));
        }
    } else {
        for syllable in syllables {
            words.push(PinyinWord::new(segment.text.clone(), syllable.to_string()));
        }
    }
}

fn name_segments(input: &str) -> Vec<Segment> {
    let Some(prefix) = lexicon().longest_surname_prefix(input) else {
        return default_matcher().segments(input);
    };

    let Some(pinyin) = lexicon().surname_pinyin(prefix) else {
        return default_matcher().segments(input);
    };

    let mut segments = vec![Segment {
        text: prefix.to_string(),
        pinyin: pinyin.to_string(),
        matched: true,
    }];
    segments.extend(plain_matcher().segments(&input[prefix.len()..]));
    segments
}

fn lexicon() -> &'static Lexicon {
    LEXICON.get_or_init(Lexicon::new)
}

fn default_matcher() -> &'static Matcher {
    DEFAULT_MATCHER.get_or_init(|| Matcher::new(lexicon().default_entries()))
}

fn plain_matcher() -> &'static Matcher {
    PLAIN_MATCHER.get_or_init(|| Matcher::new(lexicon().plain_entries()))
}

fn surname_matcher() -> &'static Matcher {
    SURNAME_MATCHER.get_or_init(|| Matcher::new(lexicon().surname_entries()))
}

fn is_punctuation_token(token: &str) -> bool {
    token.chars().all(|ch| !ch.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn converts_with_longest_dictionary_matches() {
        assert_eq!(convert("你好世界"), ["nǐ hǎo", "shì jiè"]);
        assert_eq!(
            convert("中国人喜欢中国吃饭"),
            ["zhōng guó rén", "xǐ huan", "zhōng guó", "chī fàn"]
        );
    }

    #[test]
    fn keeps_unmatched_text_as_tokens() {
        assert_eq!(convert("Hi!"), ["H", "i", "!"]);
    }

    #[test]
    fn sentence_splits_words_into_syllables() {
        assert_eq!(
            Pinyin::sentence("你好，世界").to_string(),
            "nǐ hǎo ， shì jiè"
        );
        assert_eq!(Pinyin::phrase("你好，世界").to_string(), "nǐ hǎo shì jiè");
    }

    #[test]
    fn formats_tone_styles_correctly() {
        assert_eq!(
            Converter::new("你好")
                .with_tone_style(ToneStyle::Number)
                .to_string(),
            "ni3 hao3"
        );
        assert_eq!(Converter::new("旅行").to_string(), "lǚ xíng");
        assert_eq!(Converter::new("旅行").without_tone().to_string(), "lv xing");
    }

    #[test]
    fn handles_names_and_passports() {
        assert_eq!(Pinyin::name("单某某").to_string(), "shàn mǒu mǒu");
        assert_eq!(Pinyin::name("单于单").to_string(), "chán yú dān");
        assert_eq!(Pinyin::passport_name("吕秀才").to_string(), "lyu xiu cai");
    }

    #[test]
    fn builds_permalink_and_abbr() {
        assert_eq!(
            Pinyin::permalink("带着希望去旅行"),
            "dai-zhe-xi-wang-qu-lv-xing"
        );
        assert_eq!(
            Pinyin::permalink_with("带着希望去旅行", "_").unwrap(),
            "dai_zhe_xi_wang_qu_lv_xing"
        );
        assert!(Pinyin::permalink_with("你好", "=").is_err());
        assert_eq!(Pinyin::abbr("北京大学").to_string(), "b j d x");
        assert_eq!(Pinyin::name_abbr("单某某").to_string(), "s m m");
    }

    #[test]
    fn supports_configured_conversion() {
        let no_words = PinyinConfig::new().with_long_words(false);
        assert_eq!(
            convert_with_config("你好", &no_words).unwrap(),
            ["nǐ", "hǎo"]
        );

        let err = convert_with_config("你好", &PinyinConfig::new().with_max_length(1));
        assert!(matches!(err, Err(PinyinError::InputTooLong { .. })));
    }

    #[test]
    fn exposes_chars_and_heteronyms() {
        assert_eq!(Pinyin::chars("重庆").to_string(), "zhòng qìng");
        assert!(Pinyin::heteronym("重").iter().any(|(_, items)| {
            items.contains(&"zhòng".to_string()) && items.contains(&"chóng".to_string())
        }));
    }
}
