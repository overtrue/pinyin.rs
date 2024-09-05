use std::env;
use std::fs::{write, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

const DATA_PATH: &str = "data";
const PINYIN_CHARS_FILE: &str = "chars.txt";
const PINYIN_WORDS_FILE: &str = "words.txt";
const PINYIN_SURNAMES_FILE: &str = "surnames.txt";
const PINYIN_HETERONYMS_FILE: &str = "heteronyms.txt";

fn main() {
    init();
    generate_chars();
    // generate_words();
    // generate_surnames();
    // generate_heteronyms();
}

fn init() {
    // current directory
    // create target files
    for file in [
        PINYIN_CHARS_FILE,
        PINYIN_WORDS_FILE,
        PINYIN_SURNAMES_FILE,
        PINYIN_HETERONYMS_FILE,
    ] {
        let path = Path::new("./data").join(file);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)
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
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_PATH).join(PINYIN_CHARS_FILE))
        .unwrap();

    for (chinese, pinyin) in data.iter() {
        writeln!(
            file,
            "{}: {}",
            chinese,
            pinyin
        ).expect("Failed to write chars to file");
    }
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
    let mut file = OpenOptions::new()
        .truncate(true)
        .open(Path::new(DATA_PATH).join(PINYIN_WORDS_FILE))
        .unwrap();

    for (chinese, pinyin) in data.iter() {
        writeln!(
            file,
            "{}: {}",
            chinese,
            pinyin
        ).expect("Failed to write words to file");
    }
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
    let mut file = OpenOptions::new()
        .append(true)
        .open(Path::new(DATA_PATH).join(PINYIN_SURNAMES_FILE))
        .unwrap();

    for (chinese, pinyin) in data.iter() {
        writeln!(
            file,
            "{}: {}",
            chinese,
            pinyin
        ).expect("Failed to write surnames to file");
    }
}

fn generate_heteronyms() {
    // contents: "重,好....."
    let mut file = File::open(Path::new("sources/heteronyms.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data = contents.split(',').collect::<Vec<&str>>();

    // 将结果写入文件
    let mut file = OpenOptions::new()
        .append(true)
        .open(Path::new(DATA_PATH).join(PINYIN_HETERONYMS_FILE))
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
    // U+41F8: chéng tīng  # 䇸
    // 顶证: dǐng zhèng
    // 燕: yān
    if parts.len() == 2 && !parts[0].starts_with("#") {
        let chinese = parts[0].trim().to_string();
        let mut pinyin = parts[1]
            .split_whitespace()
            .take_while(|s| !s.starts_with("#"))
            .collect::<Vec<&str>>().join(" ");

        assert!(chinese.len() >= 1 && pinyin.len() >= 1);

        data.push((chinese, pinyin.trim().parse().unwrap()))
    }
}
