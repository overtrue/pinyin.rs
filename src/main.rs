use std::io::{self, IsTerminal, Read};
use std::process;

use clap::{Parser, ValueEnum, crate_version};
use pinyin::{Converter, Pinyin, ToneStyle, YuStyle};

#[derive(Debug, Parser)]
#[command(
    name = "pinyin",
    version = crate_version!(),
    about = "Fast Chinese to Pinyin conversion",
    after_help = "Examples:\n  pinyin \"你好世界\"\n  pinyin -t number \"你好\"\n  pinyin --permalink \"中华人民共和国\"\n  pinyin --abbr \"北京大学\"\n  pinyin --name \"单某某\"\n  echo \"中国\" | pinyin"
)]
struct Args {
    #[arg(value_name = "TEXT", trailing_var_arg = true)]
    text: Vec<String>,

    #[arg(short = 't', long = "tone", value_enum, default_value_t = ToneArg::Mark)]
    tone: ToneArg,

    #[arg(short = 'y', long = "yu", value_enum, default_value_t = YuArg::Umlaut)]
    yu: YuArg,

    #[arg(short = 'f', long)]
    flatten: bool,

    #[arg(short = 's', long)]
    surname: bool,

    #[arg(short = 'c', long = "chinese-only")]
    chinese_only: bool,

    #[arg(short = 'p', long)]
    permalink: bool,

    #[arg(short = 'a', long)]
    abbr: bool,

    #[arg(short = 'n', long)]
    name: bool,

    #[arg(long = "name-abbr")]
    name_abbr: bool,

    #[arg(long)]
    passport: bool,

    #[arg(long, default_value = " ")]
    separator: String,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ToneArg {
    Mark,
    Number,
    None,
}

impl From<ToneArg> for ToneStyle {
    fn from(value: ToneArg) -> Self {
        match value {
            ToneArg::Mark => Self::Mark,
            ToneArg::Number => Self::Number,
            ToneArg::None => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum YuArg {
    Umlaut,
    V,
    Yu,
    U,
}

impl From<YuArg> for YuStyle {
    fn from(value: YuArg) -> Self {
        match value {
            YuArg::Umlaut => Self::Umlaut,
            YuArg::V => Self::V,
            YuArg::Yu => Self::Yu,
            YuArg::U => Self::U,
        }
    }
}

#[derive(Debug)]
struct Config {
    text: String,
    tone_style: ToneStyle,
    yu_style: YuStyle,
    flatten: bool,
    surname_mode: bool,
    chinese_only: bool,
    permalink: bool,
    abbr: bool,
    name_mode: bool,
    name_abbr: bool,
    passport: bool,
    separator: String,
}

impl Config {
    fn from_args(args: Args) -> Result<Self, String> {
        let text = read_text(args.text)?;

        Ok(Self {
            text,
            tone_style: args.tone.into(),
            yu_style: args.yu.into(),
            flatten: args.flatten,
            surname_mode: args.surname,
            chinese_only: args.chinese_only,
            permalink: args.permalink,
            abbr: args.abbr,
            name_mode: args.name,
            name_abbr: args.name_abbr,
            passport: args.passport,
            separator: args.separator,
        })
    }
}

fn main() {
    let config = match Config::from_args(Args::parse()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            process::exit(2);
        }
    };

    println!("{}", process_text(&config));
}

fn read_text(parts: Vec<String>) -> Result<String, String> {
    if !parts.is_empty() {
        return Ok(parts.join(" "));
    }

    if io::stdin().is_terminal() {
        return Err("missing text; pass TEXT or pipe input on stdin".to_string());
    }

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|error| format!("failed to read stdin: {error}"))?;

    let text = buffer.trim().to_string();
    if text.is_empty() {
        return Err("input text is empty".to_string());
    }

    Ok(text)
}

fn process_text(config: &Config) -> String {
    if config.permalink {
        return Pinyin::permalink(&config.text);
    }

    if config.abbr {
        return Pinyin::abbr(&config.text).to_string_with(&config.separator);
    }

    if config.name_abbr {
        return Pinyin::name_abbr(&config.text).to_string_with(&config.separator);
    }

    if config.passport {
        return Pinyin::passport_name(&config.text).to_string_with(&config.separator);
    }

    if config.name_mode {
        return Pinyin::name(&config.text)
            .with_tone_style(config.tone_style)
            .to_string_with(&config.separator);
    }

    let mut converter = Converter::new(&config.text).with_tone_style(config.tone_style);
    converter = match config.yu_style {
        YuStyle::Umlaut => converter.yu_to_umlaut(),
        YuStyle::V => converter.yu_to_v(),
        YuStyle::Yu => converter.yu_to_yu(),
        YuStyle::U => converter.yu_to_u(),
    };

    if config.flatten {
        converter = converter.flatten();
    }
    if config.surname_mode {
        converter = converter.as_surnames();
    }
    if config.chinese_only {
        converter = converter.only_hans();
    }

    converter.to_string_with(&config.separator)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_config(text: &str) -> Config {
        Config {
            text: text.to_string(),
            tone_style: ToneStyle::Mark,
            yu_style: YuStyle::Umlaut,
            flatten: false,
            surname_mode: false,
            chinese_only: false,
            permalink: false,
            abbr: false,
            name_mode: false,
            name_abbr: false,
            passport: false,
            separator: " ".to_string(),
        }
    }

    #[test]
    fn converts_basic_text() {
        assert_eq!(process_text(&base_config("你好世界")), "nǐ hǎo shì jiè");
    }

    #[test]
    fn converts_tone_numbers() {
        let mut config = base_config("你好");
        config.tone_style = ToneStyle::Number;
        assert_eq!(process_text(&config), "ni3 hao3");
    }

    #[test]
    fn converts_special_modes() {
        let mut config = base_config("带着希望去旅行");
        config.permalink = true;
        assert_eq!(process_text(&config), "dai-zhe-xi-wang-qu-lv-xing");

        let mut config = base_config("北京大学");
        config.abbr = true;
        assert_eq!(process_text(&config), "b j d x");

        let mut config = base_config("单某某");
        config.name_mode = true;
        assert_eq!(process_text(&config), "shàn mǒu mǒu");
    }
}
