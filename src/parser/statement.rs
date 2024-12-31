use super::{expression::Expression, Parse, ParseError, Parser};

use crate::lexer::{Location, TokenKind};

// 6.8

#[derive(Debug)]
pub enum UnlabeledStatement {
    JumpStatement(JumpStatement),
}

impl Parse for UnlabeledStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let jump_statement = JumpStatement::parse(parser)?;
        Ok(UnlabeledStatement::JumpStatement(jump_statement))
    }
}

// 6.8.2
#[derive(Debug)]
pub struct CompoundStatement {
    pub block_items: Vec<BlockItem>,
}

impl Parse for CompoundStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(TokenKind::LBrace)?;
        let block_items = parser.many1()?;
        parser.expect(TokenKind::RBrace)?;
        Ok(CompoundStatement { block_items })
    }
}

#[derive(Debug)]
pub enum BlockItem {
    UnlabeledStatement(UnlabeledStatement),
}

impl Parse for BlockItem {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let unlabeled_statement = UnlabeledStatement::parse(parser)?;
        Ok(BlockItem::UnlabeledStatement(unlabeled_statement))
    }
}

// 6.8.6
#[derive(Debug)]
pub enum JumpStatement {
    Return {
        location: Location,
        expression: Expression,
    },
}

impl Parse for JumpStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let location = parser.expect(TokenKind::Return)?.location;
        let expression = Expression::parse(parser)?;
        parser.expect(TokenKind::SemiColon)?;
        Ok(JumpStatement::Return {
            location,
            expression,
        })
    }
}
