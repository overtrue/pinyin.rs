use anyhow::Error;
use glob::glob;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> Result<(), Error> {
    let mut data = vec![];

    for entey in glob("sources/**/*.txt")? {
        let path = entey?;
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for line in contents.lines() {
            parse_line(line, &mut data);
        }
    }

    // 将结果写入文件
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file = File::create(Path::new(&out_dir).join("__words.rs")).unwrap();
    writeln!(
        file,
        "const INCLUDE_WORDS: [(&str, &str); {}] = {:#?};",
        data.len(),
        data
    )
    .unwrap();

    Ok(())
}

fn parse_line(line: &str, data: &mut Vec<(String, String)>) {
    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        let chinese = parts[0].to_string();
        let pinyin = parts[1].to_string();
        data.push((chinese, pinyin))
    }
}
