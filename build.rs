use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

const PINYIN_WORDS_FILE: &str = "__pinyin_words.rs";

fn main() {
    init();
    generate_words();
}

fn init() {
    let out_dir = env::var("OUT_DIR").unwrap();
    File::create(Path::new(&out_dir).join(PINYIN_WORDS_FILE)).unwrap();
}

fn generate_words() {
    let mut data = vec![];

    for path in [
        Path::new("sources/words.txt"),
        Path::new("sources/pathes/words.txt"),
    ] {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            parse_line(line, &mut data);
        }
    }

    // 将结果写入文件
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .open(Path::new(&out_dir).join(PINYIN_WORDS_FILE))
        .unwrap();
    writeln!(
        file,
        "const PINYIN_WORDS: [(&str, &str); {}] = {:#?};",
        data.len(),
        data
    )
    .unwrap();
}

fn parse_line(line: &str, data: &mut Vec<(String, String)>) {
    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        let chinese = parts[0].to_string();
        let pinyin = parts[1].to_string();
        data.push((chinese, pinyin))
    }
}
