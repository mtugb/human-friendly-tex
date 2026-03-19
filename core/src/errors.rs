#[derive(thiserror::Error, Debug)]
#[error("Error at line {line}, col {col}: {kind}")]
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub kind: ParseErrorKind, // さっき定義した Enum をここに入れる
}

#[derive(thiserror::Error, Debug)]
pub enum ParseErrorKind {
    #[error("Indentation mismatch: expected {expected}, found {found}")]
    Indent { expected: usize, found: usize },

    #[error("Internal stack error: {0}")]
    Stack(String), // fold_stackからのエラーメッセージなどを入れる

    #[error("Unknown command: {0}")]
    UnknownCommand(String),
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Unknown field: {0}")]
    UnknownField(String),
    #[error("Invalid value for field \"{field_name}\": {reason}")]
    Value { field_name: String, reason: String },
    #[error("Invalid regex in field \"{field_name}\": {source}")]
    Regex {
        field_name: String,
        #[from()]
        source: regex::Error,
    },
}
