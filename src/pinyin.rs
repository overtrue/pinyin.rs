use crate::{ToneStyle, YuStyle};

pub(crate) fn format_phrase(phrase: &str, tone_style: ToneStyle, yu_style: YuStyle) -> String {
    phrase
        .split_whitespace()
        .map(|syllable| format_syllable(syllable, tone_style, yu_style))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn format_syllable(syllable: &str, tone_style: ToneStyle, yu_style: YuStyle) -> String {
    match tone_style {
        ToneStyle::Mark if yu_style == YuStyle::Umlaut => syllable.to_string(),
        ToneStyle::Mark => render_base(&normalize(syllable).0, yu_style),
        ToneStyle::Number => {
            let (base, tone) = normalize(syllable);
            let mut rendered = render_base(&base, yu_style);
            if (1..=4).contains(&tone) {
                rendered.push(char::from_digit(tone as u32, 10).expect("tone is a digit"));
            }
            rendered
        }
        ToneStyle::None => render_base(&normalize(syllable).0, yu_style),
    }
}

pub(crate) fn first_pronunciation(phrase: &str) -> &str {
    phrase.split_whitespace().next().unwrap_or(phrase)
}

pub(crate) fn split_phrase(phrase: &str) -> Vec<&str> {
    phrase.split_whitespace().collect()
}

pub(crate) fn initials_token(token: &str) -> String {
    let plain = format_phrase(token, ToneStyle::None, YuStyle::V);
    let cleaned = plain
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>();

    if cleaned.chars().any(|ch| ch.is_ascii_digit()) {
        return cleaned;
    }

    cleaned
        .chars()
        .next()
        .map(|ch| ch.to_string())
        .unwrap_or_default()
}

pub(crate) fn slug_token(token: &str) -> String {
    format_phrase(token, ToneStyle::None, YuStyle::V)
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
}

fn normalize(syllable: &str) -> (String, u8) {
    let mut tone = 5;
    let mut base = String::with_capacity(syllable.len());

    for ch in syllable.chars() {
        let (plain, mark_tone) = match ch {
            'ā' => ('a', 1),
            'á' => ('a', 2),
            'ǎ' => ('a', 3),
            'à' => ('a', 4),
            'ē' => ('e', 1),
            'é' => ('e', 2),
            'ě' => ('e', 3),
            'è' => ('e', 4),
            'ī' => ('i', 1),
            'í' => ('i', 2),
            'ǐ' => ('i', 3),
            'ì' => ('i', 4),
            'ō' => ('o', 1),
            'ó' => ('o', 2),
            'ǒ' => ('o', 3),
            'ò' => ('o', 4),
            'ū' => ('u', 1),
            'ú' => ('u', 2),
            'ǔ' => ('u', 3),
            'ù' => ('u', 4),
            'ǖ' => ('ü', 1),
            'ǘ' => ('ü', 2),
            'ǚ' => ('ü', 3),
            'ǜ' => ('ü', 4),
            _ => {
                base.push(ch);
                continue;
            }
        };
        tone = mark_tone;
        base.push(plain);
    }

    (base, tone)
}

fn render_base(base: &str, yu_style: YuStyle) -> String {
    match yu_style {
        YuStyle::Umlaut => base.to_string(),
        YuStyle::V => base.replace('ü', "v"),
        YuStyle::Yu => base.replace('ü', "yu"),
        YuStyle::U => base.replace('ü', "u"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_tone_numbers_at_the_end() {
        assert_eq!(
            format_syllable("hǎo", ToneStyle::Number, YuStyle::V),
            "hao3"
        );
        assert_eq!(format_syllable("lǚ", ToneStyle::Number, YuStyle::V), "lv3");
    }

    #[test]
    fn keeps_umlaut_for_mark_style_by_default() {
        assert_eq!(
            format_syllable("lǚ", ToneStyle::Mark, YuStyle::Umlaut),
            "lǚ"
        );
        assert_eq!(format_syllable("lǚ", ToneStyle::None, YuStyle::Yu), "lyu");
    }

    #[test]
    fn builds_slug_tokens() {
        assert_eq!(slug_token("lǚ"), "lv");
        assert_eq!(slug_token("Hello!"), "hello");
    }
}
