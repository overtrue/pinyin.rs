use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

const PINYIN_CHARS_FILE: &str = "__pinyin_chars.rs";
const PINYIN_WORDS_FILE: &str = "__pinyin_words.rs";
const PINYIN_SURNAMES_FILE: &str = "__pinyin_surnames.rs";
const PINYIN_HETERONYMS_FILE: &str = "__pinyin_heteronyms.rs";

fn main() {
    init();
    generate_chars();
    generate_words();
    generate_surnames();
    generate_heteronyms();
}

fn init() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // create target files
    for file in [
        PINYIN_CHARS_FILE,
        PINYIN_WORDS_FILE,
        PINYIN_SURNAMES_FILE,
        PINYIN_HETERONYMS_FILE,
    ] {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(Path::new(&out_dir).join(file))
            .unwrap();
        writeln!(file, "").expect("Failed to write to file");
    }
}

fn generate_chars() {
    let mut data = vec![];

    for path in [
        Path::new("sources/chars.txt"),
        Path::new("sources/pathes/chars.txt"),
    ]  {
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
        .open(Path::new(&out_dir).join(PINYIN_CHARS_FILE))
        .unwrap();
    writeln!(
        file,
        "const PINYIN_CHARS: [(&str, &str); {}] = {:#?};",
        data.len(),
        data
    ).expect("Failed to write chars to file");
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
    .expect("Failed to write words to file");
}

fn generate_surnames() {
    let mut data = vec![];

    let mut file = File::open(Path::new("sources/surnames.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        parse_line(line, &mut data);
    }

    // 将结果写入文件
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .open(Path::new(&out_dir).join(PINYIN_SURNAMES_FILE))
        .unwrap();

    writeln!(
        file,
        "const PINYIN_SURNAMES: [(&str, &str); {}] = {:#?};",
        data.len(),
        data
    ).expect("Failed to write surnames to file");
}

fn generate_heteronyms() {
    // contents: "重,好....."
    let mut file = File::open(Path::new("sources/heteronyms.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data = contents.split(',').collect::<Vec<&str>>();

    // 将结果写入文件
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .open(Path::new(&out_dir).join(PINYIN_HETERONYMS_FILE))
        .unwrap();

    writeln!(
        file,
        "const PINYIN_HETERONYMS: [&str; {}] = {:#?};",
        data.len(),
        data
    ).expect("Failed to write heteronyms to file");
}


fn parse_line(line: &str, data: &mut Vec<(String, String)>) {
    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        let chinese = parts[0].to_string();
        let pinyin = parts[1].to_string();
        data.push((chinese, pinyin))
    }
}
