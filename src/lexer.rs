use ecow::EcoString;

pub struct Location {
    pub file: EcoString,
    /// 0-indexed
    pub line: usize,
    /// 0-indexed
    pub column: usize,
}

pub enum TokenKind {
    Integer(i64),
}

pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
}
