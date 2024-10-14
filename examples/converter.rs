extern crate pinyin;

use crate::pinyin::converter::Converter;

fn main() {
    // convert()
    let converter = Converter::new("你好");
    let result = converter.convert();
    output("Converter::new(\"你好\")'", result);

    // to_string()
    let converter = Converter::new("你好");
    let result = converter.to_string();
    output("Converter::new(\"你好\").to_string()", result);

    // with_tone_style()
    let mut converter = Converter::new("你好");
    let result = converter
        .with_tone_style(pinyin::ToneStyle::None)
        .to_string();
    output(
        "Converter::new(\"你好\").with_tone_style(pinyin::ToneStyle::Tone).to_string()",
        result,
    );

    // without_tone()
    let mut converter = Converter::new("你好");
    let result = converter.without_tone().to_string();
    output(
        "Converter::new(\"你好\").without_tone().to_string()",
        result,
    );

    // yu_to_yu()
    let mut converter = Converter::new("旅行");
    let result = converter.yu_to_yu().to_string();
    output("Converter::new(\"旅行\").yu_to_yu().to_string()", result);

    // yu_to_u()
    let mut converter = Converter::new("旅行");
    let result = converter.yu_to_u().to_string();
    output("Converter::new(\"旅行\").yu_to_u().to_string()", result);

    // yu_to_v()
    let mut converter = Converter::new("旅行");
    let result = converter.yu_to_v().to_string();
    output("Converter::new(\"旅行\").yu_to_v().to_string()", result);

    // flatten()
    let mut converter = Converter::new("重好");
    let result = converter.flatten().to_string();
    output("Converter::new(\"重好\").flatten().to_string()", result);

    // as_surnames()
    let mut converter = Converter::new("尉迟恭");
    let result = converter.as_surnames().to_string();
    output(
        "Converter::new(\"尉迟恭\").as_surnames().to_string()",
        result,
    );

    // no only_hans()
    let mut converter = Converter::new("你好，世界！");
    let result = converter.only_hans().convert();
    output("Converter::new(\"你好，世界！\").convert()", result);

    // only_hans()
    let mut converter = Converter::new("你好，世界！");
    let result = converter.only_hans().to_string();
    output(
        "Converter::new(\"你好，世界！\").only_hans().to_string()",
        result,
    );
}

fn output<T: std::fmt::Debug>(test: &str, result: T) {
    println!("\n");
    println!("\x1b[34m-> The result of '{}' is:\x1b[0m", test);

    println!("\x1b[32m{:#?}\x1b[0m", result);
}
