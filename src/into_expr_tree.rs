use std::{collections::HashMap, ops::Index};

use anyhow::{anyhow, Result};
use fef::v0::{
    expr::{
        Expr, ExprBinaryFloat64Literal, ExprSignedIntLiteral, ExprTree, ExprUnsignedIntLiteral,
        ExprVariable,
    },
    raw::VariableLengthEnum,
};

use crate::parser::{ParsedToken, Token};

pub(crate) fn into_expr_tree(
    tokens: &mut impl Iterator<Item = Result<ParsedToken>>,
    variables: &mut Vec<String>,
    in_parenthesis: bool,
) -> Result<ExprTree> {
    let mut symbols: Vec<Option<Symbol>> = Vec::new();

    while let Some(parsed_token) = tokens.next() {
        let parsed_token = parsed_token?;

        let symbol = match parsed_token.token {
            Token::Asterisk => Symbol::Operator(Operator::Asterisk),
            Token::Backslash => Symbol::Operator(Operator::Backslash),
            Token::Caret => Symbol::Operator(Operator::Caret),
            Token::Comma => Symbol::Operator(Operator::Comma),
            Token::DoubleSlash => Symbol::Operator(Operator::DoubleSlash),
            Token::Equals => Symbol::Operator(Operator::Equals),
            Token::GreaterThan => Symbol::Operator(Operator::GreaterThan),
            Token::GreaterThanOrEqual => Symbol::Operator(Operator::GreaterThanOrEqual),
            Token::LessThan => Symbol::Operator(Operator::LessThan),
            Token::LessThanOrEqual => Symbol::Operator(Operator::LessThanOrEqual),
            Token::Minus => Symbol::Operator(Operator::Minus),
            Token::NotEquals => Symbol::Operator(Operator::NotEquals),
            Token::Percent => Symbol::Operator(Operator::Percent),
            Token::Plus => Symbol::Operator(Operator::Plus),
            Token::Slash => Symbol::Operator(Operator::Slash),
            Token::Period => Symbol::Operator(Operator::Period),

            Token::BoolLiteral(b) => {
                let number: u64 = if b { 1 } else { 0 };
                let expr_obj: ExprUnsignedIntLiteral<ExprTree> =
                    ExprUnsignedIntLiteral::from(number);
                let expr: Expr<ExprTree> = expr_obj.into();
                Symbol::Operand(ExprTree::from(expr))
            }
            Token::FloatLiteral(f) => {
                let expr_obj = ExprBinaryFloat64Literal::from(f);
                let expr: Expr<ExprTree> = expr_obj.into();
                Symbol::Operand(ExprTree::from(expr))
            }
            Token::IntLiteral(i) => {
                let expr_obj = ExprSignedIntLiteral::from(i);
                let expr: Expr<ExprTree> = expr_obj.into();
                Symbol::Operand(ExprTree::from(expr))
            }
            Token::Identifier(name) => {
                let variable_id = if let Some(id) = variables.iter().position(|v| v == &name) {
                    id
                } else {
                    let id = variables.len();
                    variables.push(name);
                    id
                };
                let vre = VariableLengthEnum::from(variable_id);
                let expr: Expr<ExprTree> = ExprVariable::from(vre).into();
                Symbol::Operand(ExprTree::from(expr))
            }

            Token::RightParenthesis => {
                if in_parenthesis {
                    break;
                } else {
                    return Err(anyhow!(
                        "Unexpected ending parenthesis at {}",
                        parsed_token.end
                    ));
                }
            }

            Token::LeftParenthesis => {
                let expr = into_expr_tree(tokens, variables, true)?;
                Symbol::Operand(expr)
            }
        };

        symbols.push(Some(symbol));
    }

    // Unary operators + expr check - they have the highest precedence

    // Binary operators - they have the second highest precedence (no others are defined)
    todo!()
}

pub(crate) enum Symbol {
    Operand(ExprTree),
    Operator(Operator),
}

pub(crate) enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleSlash,
    Backslash,
    Percent,
    Caret,
    Comma,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Period,
}
