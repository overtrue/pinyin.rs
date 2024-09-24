use std::{cmp::PartialEq, fmt::Display, str::FromStr};

#[cfg(test)]
mod tests {
    use crate::{Pinyin, ToneStyle};
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
    }
}
