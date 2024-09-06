use rayon::{iter::*, slice::ParallelSlice};
use std::collections::HashMap;

pub trait Loader {
    fn get_chunk_words(&self, size: usize) -> Vec<HashMap<&str, &str>>;
}

#[derive(Debug, Default)]
pub struct WordsLoader {
    words: HashMap<String, String>,
}

impl Loader for WordsLoader {
    fn get_chunk_words(&self, size: usize) -> Vec<HashMap<&str, &str>> {
        assert!(size > 0);
        self.words
            .par_iter()
            .collect::<Vec<_>>()
            .par_chunks(self.words.len() / size)
            .map(|chunk| {
                chunk
                    .par_iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect()
            })
            .collect()
    }
}

impl WordsLoader {
    pub fn new() -> Self {
        let words_files = [
            include_str!("../data/words_0.txt").lines(),
            include_str!("../data/words_1.txt").lines(),
            include_str!("../data/words_2.txt").lines(),
            include_str!("../data/words_3.txt").lines(),
            include_str!("../data/words_4.txt").lines(),
            include_str!("../data/words_5.txt").lines(),
            include_str!("../data/words_6.txt").lines(),
            include_str!("../data/words_7.txt").lines(),
            include_str!("../data/words_8.txt").lines(),
            include_str!("../data/words_9.txt").lines(),
        ];

        let words = words_files
            .into_par_iter()
            .map(|lines| {
                let mut list = vec![];
                for line in lines {
                    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        let chinese = parts[0].to_string();
                        let pinyin = parts[1].to_string();
                        list.push((chinese, pinyin));
                    }
                }
                list
            })
            .flatten()
            .collect();
        Self { words }
    }
}
