use ecow::EcoString;

use crate::lexer::{Lexer, Location, TokenKind};

pub struct FunctionDefinition {
    pub location: Location,
    pub identifier: EcoString,
    pub value: i64,
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> FunctionDefinition {
        let int = self.lexer.next().unwrap();
        assert_eq!(int.kind, TokenKind::Int);
        let identifier = self.lexer.next().unwrap();
        let identifier = match identifier.kind {
            TokenKind::Identifier(ident) => ident,
            _ => panic!(),
        };
        assert_eq!(self.lexer.next().unwrap().kind, TokenKind::LParen);
        assert_eq!(self.lexer.next().unwrap().kind, TokenKind::RParen);
        assert_eq!(self.lexer.next().unwrap().kind, TokenKind::LBrace);
        assert_eq!(self.lexer.next().unwrap().kind, TokenKind::Return);
        let value = self.lexer.next().unwrap();
        let value = match value.kind {
            TokenKind::Integer(value) => value,
            _ => panic!(),
        };
        assert_eq!(self.lexer.next().unwrap().kind, TokenKind::SemiColon);
        assert_eq!(self.lexer.next().unwrap().kind, TokenKind::RBrace);

        FunctionDefinition {
            location: int.location,
            identifier,
            value,
        }
    }
}
