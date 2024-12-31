use ecow::EcoString;

#[derive(Debug, PartialEq)]
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

// 6.4
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // keywords
    Int,
    Return,
    // identifier
    Identifier(EcoString),
    // constant
    Integer(i64),
    // string-literal
    // punctuator 6.4.6 keep the order
    LParen,
    RParen,
    LBrace,
    RBrace,
    Plus,
    SemiColon,
    // Error
    Unknown(u8),
}

pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
}

pub struct Lexer {
    filename: EcoString,
    current_line: usize,
    current_column: usize,
    source: Vec<u8>,
    index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    line: usize,
    column: usize,
    index: usize,
}

impl Lexer {
    pub fn new(filename: EcoString, source: Vec<u8>) -> Self {
        Self {
            filename,
            current_line: 0,
            current_column: 0,
            source,
            index: 0,
        }
    }

    pub fn current_position(&self) -> Position {
        Position {
            line: self.current_line,
            column: self.current_column,
            index: self.index,
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.current_line = position.line;
        self.current_column = position.column;
        self.index = position.index;
    }

    pub fn current_location(&self) -> Location {
        Location {
            filename: self.filename.clone(),
            line: self.current_line,
            column: self.current_column,
        }
    }

    pub fn current_line(&self) -> &str {
        let mut line_start = self.index;
        while line_start > 0 && self.source[line_start - 1] != b'\n' {
            line_start -= 1;
        }
        let mut line_end = self.index;
        while self
            .source
            .get(line_end)
            .map(|c| *c != b'\n')
            .unwrap_or(false)
        {
            line_end += 1;
        }

        std::str::from_utf8(&self.source[line_start..line_end]).unwrap()
    }

    fn current_char(&self) -> Option<u8> {
        self.source.get(self.index).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_ascii_whitespace() {
                self.index += 1;
                if c == b'\n' {
                    self.current_line += 1;
                    self.current_column = 0;
                } else {
                    self.current_column += 1;
                }
            } else {
                break;
            }
        }
    }

    fn skip1(&mut self) {
        if let Some(c) = self.current_char() {
            self.index += 1;
            if c == b'\n' {
                self.current_line += 1;
                self.current_column = 0;
            } else {
                self.current_column += 1;
            }
        }
    }

    fn read_while(&mut self, pred: impl Fn(u8) -> bool) -> &str {
        let start = self.index;
        while let Some(c) = self.current_char() {
            if pred(c) {
                self.skip1();
            } else {
                break;
            }
        }
        std::str::from_utf8(&self.source[start..self.index]).unwrap()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let location = self.current_location();

        match self.current_char()? {
            b'0'..=b'9' => {
                let value = self.read_while(|c| c.is_ascii_digit());
                let value = value.parse().unwrap();
                Some(Token {
                    location,
                    kind: TokenKind::Integer(value),
                })
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_while(|c| c.is_ascii_alphanumeric() || c == b'_');
                let kind = match ident {
                    "int" => TokenKind::Int,
                    "return" => TokenKind::Return,
                    _ => TokenKind::Identifier(ident.into()),
                };
                Some(Token { location, kind })
            }
            b'(' => {
                self.skip1();
                Some(Token {
                    location,
                    kind: TokenKind::LParen,
                })
            }
            b')' => {
                self.skip1();
                Some(Token {
                    location,
                    kind: TokenKind::RParen,
                })
            }
            b'{' => {
                self.skip1();
                Some(Token {
                    location,
                    kind: TokenKind::LBrace,
                })
            }
            b'}' => {
                self.skip1();
                Some(Token {
                    location,
                    kind: TokenKind::RBrace,
                })
            }
            b'+' => {
                self.skip1();
                Some(Token {
                    location,
                    kind: TokenKind::Plus,
                })
            }
            b';' => {
                self.skip1();
                Some(Token {
                    location,
                    kind: TokenKind::SemiColon,
                })
            }
            c => {
                self.skip1();
                Some(Token {
                    location,
                    kind: TokenKind::Unknown(c),
                })
            }
        }
    }
}

#[test]
fn test_lexer() {
    let src = "int main() { return 42; }".as_bytes().to_vec();
    let lexer = Lexer::new("test.c".into(), src);

    let tokens: Vec<_> = lexer.collect();

    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0].kind, TokenKind::Int);
    assert_eq!(tokens[1].kind, TokenKind::Identifier("main".into()));
    assert_eq!(tokens[2].kind, TokenKind::LParen);
    assert_eq!(tokens[3].kind, TokenKind::RParen);
    assert_eq!(tokens[4].kind, TokenKind::LBrace);
    assert_eq!(tokens[5].kind, TokenKind::Return);
    assert_eq!(tokens[6].kind, TokenKind::Integer(42));
    assert_eq!(tokens[7].kind, TokenKind::SemiColon);
    assert_eq!(tokens[8].kind, TokenKind::RBrace);
}
