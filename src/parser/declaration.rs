// 6.7

use ecow::EcoString;

use crate::lexer::TokenKind;

use super::{Parse, ParseError, Parser};

#[derive(Debug)]
pub enum Declaration {
    // TODO
    NoAttr {
        // TODO
        // declaration_specifiers: DeclarationSpecifiers,
        // TODO init_declarator_list: InitDeclaratorList,
        init_declarator: EcoString,
    },
}

impl Parse for Declaration {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(TokenKind::Int)?;
        let init_declarator = parser.expect_identifier()?;
        parser.expect(TokenKind::SemiColon)?;
        Ok(Declaration::NoAttr { init_declarator })
    }
}
