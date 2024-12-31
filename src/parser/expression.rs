use crate::lexer::{Location, TokenKind};

use super::{Constant, Parse, ParseError, Parser};

// 6.5

// 6.5.1

#[derive(Debug)]
pub enum PrimaryExpression {
    Constant { value: Constant, location: Location },
}

impl Parse for PrimaryExpression {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let (location, value) = parser.expect_integer()?;
        Ok(PrimaryExpression::Constant {
            value: Constant::Integer(value),
            location,
        })
    }
}

// 6.5.2

// pub enum PostfixExpression {}

// 6.5.3

// pub enum UnaryExpression {}

// 6.5.4

// pub enum CastExpression {}

// 6.5.5

#[derive(Debug)]
pub enum MultiplicativeExpression {
    // TODO
    PrimaryExpression(PrimaryExpression),
    Mul {
        lhs: Box<MultiplicativeExpression>,
        // TODO cast
        rhs: Box<PrimaryExpression>,
        location: Location,
    },
    Div {
        lhs: Box<MultiplicativeExpression>,
        rhs: Box<PrimaryExpression>,
        location: Location,
    },
    Rem {
        lhs: Box<MultiplicativeExpression>,
        rhs: Box<PrimaryExpression>,
        location: Location,
    },
}

impl Parse for MultiplicativeExpression {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let primary_expression = PrimaryExpression::parse(parser)?;
        let mut lhs = MultiplicativeExpression::PrimaryExpression(primary_expression);
        while {
            if let Ok(t) = parser.expect(TokenKind::Asterisk) {
                let rhs = PrimaryExpression::parse(parser)?;
                lhs = MultiplicativeExpression::Mul {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    location: t.location,
                };
                true
            } else if let Ok(t) = parser.expect(TokenKind::Slash) {
                let rhs = PrimaryExpression::parse(parser)?;
                lhs = MultiplicativeExpression::Div {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    location: t.location,
                };
                true
            } else if let Ok(t) = parser.expect(TokenKind::Percent) {
                let rhs = PrimaryExpression::parse(parser)?;
                lhs = MultiplicativeExpression::Rem {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    location: t.location,
                };
                true
            } else {
                false
            }
        } {}
        Ok(lhs)
    }
}

// 6.5.6

#[derive(Debug)]
pub enum AdditiveExpression {
    // TODO
    PrimaryExpression(MultiplicativeExpression),
    Add {
        lhs: Box<AdditiveExpression>,
        rhs: Box<MultiplicativeExpression>,
        location: Location,
    },
    Minus {
        lhs: Box<AdditiveExpression>,
        rhs: Box<MultiplicativeExpression>,
        location: Location,
    },
}

impl Parse for AdditiveExpression {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let multiplicative_expression = MultiplicativeExpression::parse(parser)?;
        let mut lhs = AdditiveExpression::PrimaryExpression(multiplicative_expression);
        while {
            if let Ok(t) = parser.expect(TokenKind::Plus) {
                let rhs = MultiplicativeExpression::parse(parser)?;
                lhs = AdditiveExpression::Add {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    location: t.location,
                };
                true
            } else if let Ok(t) = parser.expect(TokenKind::Minus) {
                let rhs = MultiplicativeExpression::parse(parser)?;
                lhs = AdditiveExpression::Minus {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    location: t.location,
                };
                true
            } else {
                false
            }
        } {}
        Ok(lhs)
    }
}

// 6.5.7

// pub enum ShiftExpression {}

// 6.5.8

// pub enum RelationalExpression {}

// 6.5.9

// pub enum EqualityExpression {}

// 6.5.10

// pub enum AndExpression {}

// 6.5.11

// pub enum ExclusiveOrExpression {}

// 6.5.12

// pub enum InclusiveOrExpression {}

// 6.5.13

// pub enum LogicalAndExpression {}

// 6.5.14

// pub enum LogicalOrExpression {}

// 6.5.15

// pub enum ConditionalExpression {}

// 6.5.16

// pub enum AssignmentExpression {}

// 6.5.17

#[derive(Debug)]
pub enum Expression {
    AdditiveExpression(Box<AdditiveExpression>),
}

impl Parse for Expression {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        let additive_expression = AdditiveExpression::parse(parser)?;
        Ok(Expression::AdditiveExpression(Box::new(
            additive_expression,
        )))
    }
}
