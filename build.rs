use anyhow::{Context, Result, anyhow};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const CHUNK_COUNT: usize = 10;

fn main() -> Result<()> {
    for path in [
        "sources/chars.txt",
        "sources/patches/chars.txt",
        "sources/words.txt",
        "sources/patches/words.txt",
        "sources/surnames.txt",
        "sources/heteronyms.txt",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let data_dir = PathBuf::from(std::env::var("OUT_DIR")?).join("data");
    if data_dir.exists() {
        fs::remove_dir_all(&data_dir)
            .with_context(|| format!("failed to clean {}", data_dir.display()))?;
    }
    fs::create_dir_all(&data_dir)
        .with_context(|| format!("failed to create {}", data_dir.display()))?;

    generate_chars(&data_dir)?;
    generate_words(&data_dir)?;
    generate_surnames(&data_dir)?;
    generate_heteronyms(&data_dir)?;

    Ok(())
}

fn generate_chars(data_dir: &Path) -> Result<()> {
    let mut rows = Vec::new();
    for path in ["sources/chars.txt", "sources/patches/chars.txt"] {
        for (key, pinyin) in read_source(path)? {
            let code = key
                .strip_prefix("U+")
                .ok_or_else(|| anyhow!("expected unicode key in {path}: {key}"))?;
            let code_point = u32::from_str_radix(code, 16)
                .with_context(|| format!("invalid unicode code point {key}"))?;
            let ch = char::from_u32(code_point)
                .ok_or_else(|| anyhow!("invalid unicode scalar value {key}"))?;
            rows.push((ch.to_string(), pinyin));
        }
    }

    if rows.is_empty() {
        return Err(anyhow!("no character data found"));
    }

    write_chunks(data_dir, "chars", rows)
}

fn generate_words(data_dir: &Path) -> Result<()> {
    let mut rows = HashMap::new();
    for path in ["sources/words.txt", "sources/patches/words.txt"] {
        for (word, pinyin) in read_source(path)? {
            rows.insert(word, pinyin);
        }
    }

    if rows.is_empty() {
        return Err(anyhow!("no word data found"));
    }

    let mut rows = rows.into_iter().collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    write_chunks(data_dir, "words", rows)
}

fn generate_surnames(data_dir: &Path) -> Result<()> {
    let rows = read_source("sources/surnames.txt")?;
    if rows.is_empty() {
        return Err(anyhow!("no surname data found"));
    }

    write_lines(&data_dir.join("surnames.txt"), rows)
}

fn generate_heteronyms(data_dir: &Path) -> Result<()> {
    let content = fs::read_to_string("sources/heteronyms.txt")
        .context("failed to read sources/heteronyms.txt")?;
    let mut file =
        File::create(data_dir.join("heteronyms.txt")).context("failed to create heteronyms.txt")?;

    for item in content
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        writeln!(file, "{item}").context("failed to write heteronym data")?;
    }

    Ok(())
}

fn read_source(path: impl AsRef<Path>) -> Result<Vec<(String, String)>> {
    let path = path.as_ref();
    let content =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut rows = Vec::new();

    for line in content.lines() {
        if let Some(row) = parse_line(line) {
            rows.push(row);
        }
    }

    Ok(rows)
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let (key, value) = line.split_once(':')?;
    let pinyin = value
        .split_whitespace()
        .take_while(|part| !part.starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ");

    let key = key.trim();
    if key.is_empty() || pinyin.is_empty() {
        return None;
    }

    Some((key.to_string(), pinyin))
}

fn write_chunks(data_dir: &Path, prefix: &str, rows: Vec<(String, String)>) -> Result<()> {
    let chunk_size = rows.len().div_ceil(CHUNK_COUNT);

    for index in 0..CHUNK_COUNT {
        let start = index * chunk_size;
        let end = ((index + 1) * chunk_size).min(rows.len());
        let chunk = if start < rows.len() {
            &rows[start..end]
        } else {
            &[]
        };
        write_lines(
            &data_dir.join(format!("{prefix}_{index}.txt")),
            chunk.iter().cloned(),
        )?;
    }

    Ok(())
}

fn write_lines(path: &Path, rows: impl IntoIterator<Item = (String, String)>) -> Result<()> {
    let mut file =
        File::create(path).with_context(|| format!("failed to create {}", path.display()))?;
    for (key, pinyin) in rows {
        writeln!(file, "{key}: {pinyin}")
            .with_context(|| format!("failed to write {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_source_lines() {
        assert_eq!(
            parse_line("U+4E2D: zhōng # 中"),
            Some(("U+4E2D".to_string(), "zhōng".to_string()))
        );
        assert_eq!(parse_line("# comment"), None);
        assert_eq!(parse_line("broken"), None);
    }
}
