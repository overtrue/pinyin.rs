# Improvement Notes

## Completed

- Renamed the Cargo package to `pinyin-converter` so it can be published without conflicting with the existing `pinyin` crate. The library import remains `pinyin`, and the binary remains `pinyin`.
- Updated the project to Rust edition 2024 with MSRV `1.95`.
- Reworked the build script so generated dictionary files are written to Cargo `OUT_DIR`, not the source tree.
- Replaced the expensive full automaton startup path with longest-prefix dictionary lookup over in-memory maps.
- Fixed high-level conversion so phrase matches are split into syllables and single-character polyphones default to the first pronunciation.
- Fixed numbered tone output from forms like `ha3o` to standard `hao3`.
- Added `Pinyin`, `Converter`, `PinyinConfig`, `PinyinError`, CLI, README, API docs, CI, and release workflow coverage.

## Verified Behavior

```text
pinyin "你好世界"                  -> nǐ hǎo shì jiè
pinyin -t number "你好"            -> ni3 hao3
pinyin --permalink "带着希望去旅行" -> dai-zhe-xi-wang-qu-lv-xing
pinyin --abbr "北京大学"           -> b j d x
pinyin --name "单某某"             -> shàn mǒu mǒu
pinyin --passport "吕秀才"          -> lyu xiu cai
```

## API Cleanup

Old misspelled aliases were removed. The public API now consistently uses `Pinyin*` names.
