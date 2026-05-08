use pinyin::{Converter, Pinyin, ToneStyle};

fn main() {
    println!("=== Pinyin.rs 新 API 演示 ===\n");

    // 快捷方式
    println!("## 快捷方式\n");

    // 默认词典转换
    println!("**默认词典转换**");
    let sentence = Pinyin::sentence("带着希望去旅行");
    println!("Pinyin::sentence(\"带着希望去旅行\"):");
    println!("  .to_string() -> {}", sentence);
    println!("  .to_string_with('-') -> {}", sentence.to_string_with("-"));
    println!("  .to_permalink() -> {}", sentence.to_permalink());
    println!();

    // 生成用于链接的拼音字符串
    println!("**生成用于链接的拼音字符串**");
    let permalink = Pinyin::permalink("带着希望去旅行");
    println!("Pinyin::permalink(\"带着希望去旅行\") -> {}", permalink);
    println!();

    // 获取首字符字符串
    println!("**获取首字符字符串**");
    let abbr = Pinyin::abbr("带着希望去旅行");
    println!("Pinyin::abbr(\"带着希望去旅行\"):");
    println!("  .to_string() -> {}", abbr);
    println!("  .to_permalink() -> {}", abbr.to_permalink());
    println!();

    // 姓名首字母
    println!("**姓名首字母**");
    let name_abbr = Pinyin::name_abbr("单单单");
    println!("Pinyin::name_abbr(\"单单单\").to_string() -> {}", name_abbr);
    println!();

    // 姓名转换
    println!("**姓名转换**");
    let name = Pinyin::name("单某某");
    println!("Pinyin::name(\"单某某\").to_string() -> {}", name);
    println!();

    // 护照姓名转换
    println!("**护照姓名转换**");
    let passport_name1 = Pinyin::passport_name("吕小布");
    let passport_name2 = Pinyin::passport_name("女小花");
    println!(
        "Pinyin::passport_name(\"吕小布\").to_string() -> {}",
        passport_name1
    );
    println!(
        "Pinyin::passport_name(\"女小花\").to_string() -> {}",
        passport_name2
    );
    println!();

    // 基础 API
    println!("## 基础 API\n");

    // 默认单字模式
    println!("**默认单字模式**");
    let converter_result = Converter::new("你好，世界").convert();
    println!(
        "Converter::new(\"你好，世界\").convert() -> {:?}",
        converter_result.words()
    );
    println!();

    // 段落模式
    println!("**段落模式**");
    println!(
        "Converter::new(\"你好，世界\").to_string() -> {}",
        Converter::new("你好，世界")
    );
    println!(
        "Converter::new(\"你好，世界\").to_string_with('-') -> {}",
        Converter::new("你好，世界").to_string_with("-")
    );
    println!(
        "Converter::new(\"你好，世界\").to_permalink() -> {}",
        Converter::new("你好，世界").to_permalink()
    );
    println!();

    // 设置输出格式
    println!("**设置输出格式**");
    let input = "你好，世界";
    println!("输入: \"{}\"", input);
    println!("  默认 (符号声调): {}", Converter::new(input));
    println!(
        "  数字声调: {}",
        Converter::new(input).with_tone_style(ToneStyle::Number)
    );
    println!("  无声调: {}", Converter::new(input).without_tone());
    println!();

    // 多音字时仅取第一个读音
    println!("**多音字时仅取第一个读音**");
    println!(
        "Converter::new(\"重\").convert().to_string() -> {}",
        Converter::new("重").convert()
    );
    println!(
        "Converter::new(\"重\").flatten().convert().to_string() -> {}",
        Converter::new("重").flatten().convert()
    );
    println!();

    // 姓氏转换
    println!("**姓氏转换**");
    println!(
        "Converter::new(\"单某某\").as_surnames().to_string() -> {}",
        Converter::new("单某某").as_surnames()
    );
    println!();

    // v/yu/ü 的问题
    println!("## v/yu/ü 的问题\n");
    let travel = "旅行";
    println!("输入: \"{}\"", travel);
    println!("  默认 (带声调 lǚ / 无声调 lv): {}", Converter::new(travel));
    println!("  无声调 lv: {}", Converter::new(travel).without_tone());
    println!("  yu 格式: {}", Converter::new(travel).yu_to_yu());
    println!("  u 格式: {}", Converter::new(travel).yu_to_u());
    println!("  v 格式: {}", Converter::new(travel).yu_to_v());
    println!();

    // 链式调用示例
    println!("## 链式调用示例\n");
    let complex_result = Converter::new("你好世界123")
        .without_tone()
        .yu_to_yu()
        .flatten()
        .only_hans()
        .to_string_with("-");
    println!("复杂链式调用:");
    println!("Converter::new(\"你好世界123\")");
    println!("  .without_tone()");
    println!("  .yu_to_yu()");
    println!("  .flatten()");
    println!("  .only_hans()");
    println!("  .to_string_with(\"-\")");
    println!("结果: {}", complex_result);
}
