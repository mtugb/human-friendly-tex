use serde::Deserialize;

// RAW_CONFIG
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")] // "type" フィールドの値で分岐
pub enum CommandConfig {
    #[serde(rename = "Template")]
    Template(TemplateConfig),

    #[serde(rename = "Regex")]
    Regex(RegexConfig),

    #[serde(rename = "Environment")]
    Env(EnvConfig),
}

#[derive(Debug, Deserialize, Clone)]
pub struct TemplateConfig {
    pub pattern: String,
    pub template: String,
    pub args_count: usize,
    pub alias: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RegexConfig {
    pub pattern: String,
    pub template: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvConfig {
    pub pattern: String,
    pub env_name: String,
    pub output_prefix: Option<String>,
    pub output_suffix: Option<String>,
    pub line_prefix: Option<String>,
    pub line_suffix: Option<String>,
    pub alias: Option<Vec<String>>,
    #[serde(default)] // 何もなければString::new()つまり""が入る
    pub row_separator: String,
    #[serde(default)] // 何もなければString::new()つまり""が入る
    pub col_separator: String,
}
