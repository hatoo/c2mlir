use ecow::EcoString;

pub struct Location {
    pub filename: EcoString,
    /// 0-indexed
    pub line: usize,
    /// 0-indexed
    pub column: usize,
}

impl Location {
    pub fn mlir_location<'c>(&self, context: &'c melior::Context) -> melior::ir::Location<'c> {
        melior::ir::Location::new(context, &self.filename, self.line + 1, self.column + 1)
    }
}

pub enum TokenKind {
    Integer(i64),
}

pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
}
