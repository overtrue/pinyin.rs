use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Segment {
    pub(crate) text: String,
    pub(crate) pinyin: String,
    pub(crate) matched: bool,
}

#[derive(Clone)]
pub(crate) struct Matcher {
    entries: HashMap<String, String>,
    max_chars: usize,
}

impl Matcher {
    pub(crate) fn new(entries: Vec<(String, String)>) -> Self {
        let max_chars = entries
            .iter()
            .map(|(word, _)| word.chars().count())
            .max()
            .unwrap_or(1);
        let entries = entries.into_iter().collect();

        Self { entries, max_chars }
    }

    pub(crate) fn segments(&self, input: &str) -> Vec<Segment> {
        if input.is_empty() {
            return Vec::new();
        }

        let bounds = char_bounds(input);
        let char_count = bounds.len() - 1;
        let mut segments = Vec::with_capacity(char_count);
        let mut index = 0;

        while index < char_count {
            let max_len = self.max_chars.min(char_count - index);
            let mut matched = None;

            for len in (1..=max_len).rev() {
                let text = &input[bounds[index]..bounds[index + len]];
                if let Some(pinyin) = self.entries.get(text) {
                    matched = Some((len, text, pinyin.as_str()));
                    break;
                }
            }

            if let Some((len, text, pinyin)) = matched {
                segments.push(Segment {
                    text: text.to_string(),
                    pinyin: pinyin.to_string(),
                    matched: true,
                });
                index += len;
            } else {
                let text = &input[bounds[index]..bounds[index + 1]];
                segments.push(Segment {
                    text: text.to_string(),
                    pinyin: text.to_string(),
                    matched: false,
                });
                index += 1;
            }
        }

        segments
    }
}

fn char_bounds(input: &str) -> Vec<usize> {
    let mut bounds = input
        .char_indices()
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    bounds.push(input.len());
    bounds
}

pub(crate) fn group_unmatched_for_sentence(segments: Vec<Segment>) -> Vec<Segment> {
    let mut grouped = Vec::with_capacity(segments.len());
    let mut buffer = String::new();

    for segment in segments {
        if segment.matched {
            flush_buffer(&mut grouped, &mut buffer);
            grouped.push(segment);
        } else if segment.text.chars().all(char::is_whitespace) {
            flush_buffer(&mut grouped, &mut buffer);
        } else if segment.text.chars().all(is_cjk_punctuation) {
            flush_buffer(&mut grouped, &mut buffer);
            grouped.push(segment);
        } else {
            buffer.push_str(&segment.text);
        }
    }

    flush_buffer(&mut grouped, &mut buffer);
    grouped
}

fn flush_buffer(segments: &mut Vec<Segment>, buffer: &mut String) {
    if buffer.is_empty() {
        return;
    }

    let text = std::mem::take(buffer);
    segments.push(Segment {
        pinyin: text.clone(),
        text,
        matched: false,
    });
}

fn is_cjk_punctuation(ch: char) -> bool {
    matches!(
        ch,
        '。' | '，'
            | '、'
            | '；'
            | '：'
            | '？'
            | '！'
            | '（'
            | '）'
            | '《'
            | '》'
            | '「'
            | '」'
            | '『'
            | '』'
            | '【'
            | '】'
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_leftmost_longest_matches() {
        let matcher = Matcher::new(vec![
            ("中".to_string(), "zhōng".to_string()),
            ("中国".to_string(), "zhōng guó".to_string()),
            ("中国人".to_string(), "zhōng guó rén".to_string()),
        ]);

        let segments = matcher.segments("中国人");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].pinyin, "zhōng guó rén");
    }

    #[test]
    fn preserves_unmatched_characters() {
        let matcher = Matcher::new(vec![("中国".to_string(), "zhōng guó".to_string())]);
        let segments = matcher.segments("Hi中国!");
        let texts = segments
            .into_iter()
            .map(|segment| segment.text)
            .collect::<Vec<_>>();
        assert_eq!(texts, ["H", "i", "中国", "!"]);
    }

    #[test]
    fn groups_ascii_runs_for_sentence_output() {
        let segments = vec![
            Segment {
                text: "H".to_string(),
                pinyin: "H".to_string(),
                matched: false,
            },
            Segment {
                text: "i".to_string(),
                pinyin: "i".to_string(),
                matched: false,
            },
            Segment {
                text: "，".to_string(),
                pinyin: "，".to_string(),
                matched: false,
            },
        ];

        let grouped = group_unmatched_for_sentence(segments);
        assert_eq!(grouped[0].text, "Hi");
        assert_eq!(grouped[1].text, "，");
    }
}
