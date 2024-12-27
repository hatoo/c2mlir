use std::fmt::Display;

use ecow::EcoString;

use crate::lexer::{Lexer, Location, Token, TokenKind};

// ASTs are followed by C Standard but hierarchy may be flattened for simplicity
// Don't define tuple struct unless the number of fields is 1

// 6.4.4

#[derive(Debug)]
pub enum Constant {
    Integer(i64),
}

// 6.5

#[derive(Debug)]
pub enum Expression {
    // primary-expression
    Constant { value: Constant, location: Location },
}

// 6.8

#[derive(Debug)]
pub enum UnlabeledStatement {
    JumpStatement(JumpStatement),
}

// 6.8.2
#[derive(Debug)]
pub struct CompoundStatement {
    pub block_items: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    UnlabeledStatement(UnlabeledStatement),
}

// 6.8.6
#[derive(Debug)]
pub enum JumpStatement {
    Return {
        location: Location,
        expression: Expression,
    },
}

// 6.9

#[derive(Debug)]
pub struct TranslationUnit(pub Vec<ExternalDeclaration>);

#[derive(Debug)]
pub enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
}

// 6.9.1

#[derive(Debug)]
pub struct FunctionDefinition {
    pub location: Location,
    pub identifier: EcoString,
    pub body: CompoundStatement,
}

pub struct Parser {
    lexer: Lexer,
}

#[derive(Debug)]
pub struct ParseError {
    pub location: Location,
    pub line: String,
    // I gave up to define custom error type
    pub message: String,
}

pub trait Parse
where
    Self: Sized,
{
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
}

impl ParseError {
    pub fn new(location: Location, line: String, message: String) -> Self {
        Self {
            location,
            line,
            message,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}:{}:{}:",
            self.location.filename,
            // To 1-indexed
            self.location.line + 1,
            self.location.column + 1,
        )?;
        writeln!(f, "{}", self.line)?;
        // wtf?
        writeln!(f, "{:1$}^", "", self.location.column)?;
        writeln!(f, "{}", self.message)?;

        Ok(())
    }
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    fn error(&self, message: String) -> ParseError {
        // FIXME currently it points to the next token
        ParseError::new(
            self.lexer.current_location(),
            self.lexer.current_line().to_string(),
            message,
        )
    }

    pub fn expect(&mut self, token_kind: TokenKind) -> Result<Token, ParseError> {
        match self.lexer.next() {
            Some(token) => {
                if token.kind == token_kind {
                    Ok(token)
                } else {
                    Err(self.error(format!("expected {:?}, found {:?}", token_kind, token.kind)))
                }
            }
            None => Err(self.error(format!("expected {:?}, found EOF", token_kind))),
        }
    }

    pub fn expect_integer(&mut self) -> Result<(Location, i64), ParseError> {
        match self.lexer.next() {
            Some(token) => match token.kind {
                TokenKind::Integer(value) => Ok((token.location, value)),
                _ => Err(self.error(format!("expected integer, found {:?}", token.kind))),
            },
            None => Err(self.error("expected integer, found EOF".to_string())),
        }
    }

    pub fn expect_identifier(&mut self) -> Result<EcoString, ParseError> {
        match self.lexer.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(identifier) => Ok(identifier),
                _ => Err(self.error(format!("expected identifier, found {:?}", token.kind))),
            },
            None => Err(self.error("expected identifier, found EOF".to_string())),
        }
    }

    pub fn expect_eof(&mut self) -> Result<(), ParseError> {
        match self.lexer.next() {
            Some(token) => Err(self.error(format!("expected EOF, found {:?}", token.kind))),
            None => Ok(()),
        }
    }

    pub fn many1<P: Parse>(&mut self) -> Result<Vec<P>, ParseError> {
        let mut items = Vec::new();
        items.push(P::parse(self)?);
        while {
            let pos = self.lexer.current_position();
            match P::parse(self) {
                Ok(item) => {
                    items.push(item);
                    true
                }
                Err(_) => {
                    self.lexer.set_position(pos);
                    false
                }
            }
        } {}
        Ok(items)
    }
}

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let (location, value) = parser.expect_integer()?;
        Ok(Expression::Constant {
            value: Constant::Integer(value),
            location,
        })
    }
}

impl Parse for UnlabeledStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let jump_statement = JumpStatement::parse(parser)?;
        Ok(UnlabeledStatement::JumpStatement(jump_statement))
    }
}

impl Parse for CompoundStatement {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(TokenKind::LBrace)?;
        let block_items = parser.many1()?;
        parser.expect(TokenKind::RBrace)?;
        Ok(CompoundStatement { block_items })
    }
}

impl Parse for BlockItem {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let unlabeled_statement = UnlabeledStatement::parse(parser)?;
        Ok(BlockItem::UnlabeledStatement(unlabeled_statement))
    }
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

impl Parse for TranslationUnit {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let external_declarations = parser.many1()?;
        parser.expect_eof()?;
        Ok(TranslationUnit(external_declarations))
    }
}

impl Parse for ExternalDeclaration {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let function_definition = FunctionDefinition::parse(parser)?;
        Ok(ExternalDeclaration::FunctionDefinition(function_definition))
    }
}

impl Parse for FunctionDefinition {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let int = parser.expect(TokenKind::Int)?;
        let identifier = parser.expect_identifier()?;
        parser.expect(TokenKind::LParen)?;
        parser.expect(TokenKind::RParen)?;
        let body = CompoundStatement::parse(parser)?;

        Ok(FunctionDefinition {
            location: int.location,
            identifier,
            body,
        })
    }
}
