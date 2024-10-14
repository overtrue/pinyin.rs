use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::string::ToString;

const DATA_PATH: &str = "data";
fn main() {
    if !std::env::args().any(|arg: String| arg == "--force") {
        // if path exists and is not empty
        if Path::new(DATA_PATH).exists() && std::fs::read_dir(DATA_PATH).unwrap().next().is_some() {
            println!("Please use `--force` to regenerate the data.");
            return;
        }

        println!("Regenerating data...");
    } else {
        println!("Forcing regeneration of data...");
    }

    cleanup();
    generate_chars();
    generate_words();
    generate_surnames();
    generate_heteronyms();
}

fn cleanup() {
    println!("Cleaning up data directory...");
    std::fs::remove_dir_all(DATA_PATH).unwrap_or(());
    std::fs::create_dir(DATA_PATH).expect("Failed to create data directory");
}

fn generate_chars() {
    let mut data = vec![];

    println!("Generating chars data...");
    for path in [
        Path::new("sources/chars.txt"),
        Path::new("sources/patches/chars.txt"),
    ] {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            if let Some(item) = parse_line(line) {
                data.push(item);
            }
        }
    }

    let chunk_size = data.len().div_ceil(10);

    for (count, (unicode, pinyin)) in data.iter().enumerate() {
        // unicode: "U+4E00"
        let code_point = u32::from_str_radix(&unicode[2..], 16).unwrap();

        let chunk_file_name = format!("chars_{}.txt", count / chunk_size);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(DATA_PATH).join(chunk_file_name))
            .unwrap();

        if let Some(chinese) = char::from_u32(code_point) {
            writeln!(file, "{}: {}", chinese, pinyin).expect("Failed to write chars to file");
        }
    }

    println!("Generated chars data.");
}

fn generate_words() {
    let mut data = HashMap::new();

    println!("Generating words data...");
    for path in [
        Path::new("sources/words.txt"),
        Path::new("sources/patches/words.txt"),
    ] {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            if let Some((chinese, pinyin)) = parse_line(line) {
                data.entry(chinese).or_insert(pinyin);
            }
        }
    }

    let chunk_size = data.len().div_ceil(10);

    for (count, (chinese, pinyin)) in hashmap_to_sorted_vec(data).iter().enumerate() {
        let chunk_file_name = format!("words_{}.txt", count / chunk_size);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(DATA_PATH).join(chunk_file_name))
            .unwrap();

        writeln!(file, "{}: {}", chinese, pinyin).expect("Failed to write words to file");
    }

    println!("Generated words data.");
}

fn generate_surnames() {
    let mut data = vec![];

    println!("Generating surnames data...");
    let mut file = File::open(Path::new("sources/surnames.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        if let Some(item) = parse_line(line) {
            data.push(item);
        }
    }

    // 将结果写入文件
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_PATH).join("surnames.txt"))
        .unwrap();

    for (chinese, pinyin) in data.iter() {
        writeln!(file, "{}: {}", chinese, pinyin).expect("Failed to write surnames to file");
    }

    println!("Generated surnames data.");
}

fn generate_heteronyms() {
    println!("Generating heteronyms data...");

    // contents: "重,好....."
    let mut file = File::open(Path::new("sources/heteronyms.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data = contents.split(',').collect::<Vec<&str>>();

    // 将结果写入文件
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_PATH).join("heteronyms.txt"))
        .unwrap();

    data.join("\n").lines().for_each(|line| {
        writeln!(file, "{}", line).expect("Failed to write heteronyms to file");
    });

    println!("Generated heteronyms data.");
}

fn hashmap_to_sorted_vec(map: HashMap<String, String>) -> Vec<(String, String)> {
    let mut vec: Vec<(String, String)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
    // U+41F8: chéng tīng  # 䇸
    // 顶证: dǐng zhèng
    // 燕: yān
    if parts.len() == 2 && !parts[0].starts_with('#') {
        let chinese = parts[0].trim().to_string();
        let pinyin = parts[1]
            .split_whitespace()
            .take_while(|s| !s.starts_with('#'))
            .collect::<Vec<&str>>()
            .join(" ");

        assert!(!chinese.is_empty() && !pinyin.is_empty());

        return Some((chinese, pinyin.trim().parse().unwrap()));
    }

    None
}
