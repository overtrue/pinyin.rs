use thiserror::Error;

/// 拼音库错误类型
#[derive(Error, Debug)]
pub enum PingyinError {
    /// 字符串解析错误
    #[error("Failed to parse string: {0}")]
    ParseStrError(String),

    /// 无效的声调
    #[error("Invalid tone: {0}, expected 1-5")]
    InvalidTone(u8),

    /// 无效的拼音格式
    #[error("Invalid pinyin format: {0}")]
    InvalidFormat(String),

    /// 数据加载错误
    #[error("Failed to load data: {0}")]
    DataLoadError(String),

    /// 匹配器构建错误
    #[error("Failed to build matcher: {0}")]
    MatcherBuildError(String),
}

impl PingyinError {
    /// 创建解析错误
    pub fn parse_error(input: &str) -> Self {
        Self::ParseStrError(input.to_string())
    }

    /// 创建声调错误
    pub fn tone_error(tone: u8) -> Self {
        Self::InvalidTone(tone)
    }

    /// 创建格式错误
    pub fn format_error(input: &str) -> Self {
        Self::InvalidFormat(input.to_string())
    }
}
