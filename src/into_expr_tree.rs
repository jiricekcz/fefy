use std::fmt::Display;

use anyhow::{anyhow, Result};
use fef::v0::{
    expr::{
        Expr, ExprAddition, ExprBinaryFloat64Literal, ExprDivision, ExprIntDivision, ExprModulo,
        ExprMultiplication, ExprNegation, ExprPower, ExprSignedIntLiteral, ExprSubtraction,
        ExprTree, ExprUnsignedIntLiteral, ExprVariable,
    },
    raw::VariableLengthEnum,
};

use crate::parser::{ParsedToken, Token};

pub(crate) fn into_expr_tree(
    tokens: &mut impl Iterator<Item = Result<ParsedToken>>,
    variables: &mut Vec<String>,
    in_parenthesis: bool,
) -> Result<ExprTree> {
    // Convert the tokens into symbols
    let mut symbols = into_symbols(tokens, variables, in_parenthesis)?
        .into_iter()
        .map(|s| Some(s))
        .collect::<Vec<_>>();

    // Removes all unary operators by composing them with their operands
    compose_unary_expressions(&mut symbols)?;
    // Covert infix notation to postfix notation
    let clean_symbols = symbols.into_iter().map(|o| o.expect("Cleaned")).collect();

    let postfix_symbols = shunting_yard_algorithm(clean_symbols)?;

    let expr = compose_binary_expressions(postfix_symbols.into_iter())?;

    Ok(expr)
}

/// Converts a sequence of tokens into a sequence of symbols.
fn into_symbols(
    tokens: &mut impl Iterator<Item = Result<ParsedToken>>,
    variables: &mut Vec<String>,
    in_parenthesis: bool,
) -> Result<Vec<ParsedSymbol>> {
    let mut symbols: Vec<ParsedSymbol> = Vec::new();

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
            Token::DoubleAsterisk => Symbol::Operator(Operator::DoubleAsterisk),

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

        symbols.push(ParsedSymbol {
            end: parsed_token.end,
            start: parsed_token.start,
            symbol,
        });
    }
    Ok(symbols)
}

/// Treats all operators in place of unary operators as unary operators with the same precedence and composes them with their operands, if possible.
fn compose_unary_expressions(symbols: &mut Vec<Option<ParsedSymbol>>) -> Result<()> {
    // What is expected in a sequence without unary operators
    let mut expecting = Expecting::Operand;

    let mut unary_operator_stack: Vec<ParsedOperator> = Vec::new();
    for i in 0..symbols.len() {
        let parsed_symbol = symbols[i].take().expect("Symbol vec not clean");
        match parsed_symbol.symbol {
            Symbol::Operator(op) => match expecting {
                Expecting::Operand => {
                    unary_operator_stack.push(ParsedOperator {
                        operator: op,
                        start: parsed_symbol.start,
                        end: parsed_symbol.end,
                    });
                }
                Expecting::Operator => {
                    expecting = Expecting::Operand;
                    symbols[i] = Some(parsed_symbol);
                }
            },
            Symbol::Operand(_) => match expecting {
                Expecting::Operand => {
                    let end: usize = parsed_symbol.end;
                    let mut start: usize = parsed_symbol.start;
                    let mut expr_to_wrap = match parsed_symbol.symbol {
                        Symbol::Operand(expr) => expr,
                        _ => unreachable!(),
                    };

                    for unary_operator in unary_operator_stack.iter().rev() {
                        start = unary_operator.start;
                        expr_to_wrap = wrap(expr_to_wrap, *unary_operator)?;
                    }
                    expecting = Expecting::Operator;
                    symbols[i] = Some(ParsedSymbol {
                        end,
                        start,
                        symbol: Symbol::Operand(expr_to_wrap),
                    });
                }
                Expecting::Operator => {
                    return Err(anyhow!(
                        "Expected operator at {}-{} found expression",
                        parsed_symbol.start,
                        parsed_symbol.end
                    ));
                }
            },
        }
    }

    Ok(to_cleaned_symbols(symbols))
}

/// Composes all binary expressions in the sequence of symbols
fn compose_binary_expressions(
    postfix_symbols: impl DoubleEndedIterator<Item = ParsedSymbol>,
) -> Result<ExprTree> {
    let mut stack: Vec<ExpressionInProgress> = Vec::new();

    for parsed_symbol in postfix_symbols.rev() {
        let add: Option<ExpressionInProgress> = match stack.last_mut() {
            None =>
            // No state in progress
            {
                match parsed_symbol.symbol {
                    Symbol::Operator(o) => {
                        stack.push(ExpressionInProgress::Operator {
                            operator: ParsedOperator {
                                operator: o,
                                start: parsed_symbol.start,
                                end: parsed_symbol.end,
                            },
                        });
                        None
                    }
                    _ => {
                        return Err(anyhow!(
                            "Expected operator at {}-{} found expression after converting to postfix notation",
                            parsed_symbol.start,
                            parsed_symbol.end
                        ));
                    }
                }
            }
            Some(in_progress) => match parsed_symbol.symbol {
                Symbol::Operator(o) => Some(ExpressionInProgress::Operator {
                    operator: ParsedOperator {
                        operator: o,
                        end: parsed_symbol.end,
                        start: parsed_symbol.start,
                    },
                }),
                Symbol::Operand(expr) => {
                    in_progress.add_operand(expr);
                    None
                }
            },
        };

        if let Some(add) = add {
            stack.push(add);
        }
        while let Some(ExpressionInProgress::LHS {
            operator: _,
            lhs: _,
            rhs: _,
        }) = stack.last()
        {
            let in_progress = stack.pop().expect("Stack not empty");
            let expr = in_progress.compose_into()?;
            match stack.last_mut() {
                None => return Ok(expr),
                Some(in_progress) => in_progress.add_operand(expr),
            }
        }
    }

    Err(anyhow!("Failed to compose expression, no complete expression found on composition stack. Composition stack: {:?}", stack))
}

/// Converts a sequence of symbols in infix notation to postfix notation
fn shunting_yard_algorithm(symbols: Vec<ParsedSymbol>) -> Result<Vec<ParsedSymbol>> {
    let mut output: Vec<ParsedSymbol> = Vec::new();
    let mut operator_stack: Vec<ParsedOperator> = Vec::new();

    for parsed_symbol in symbols {
        match parsed_symbol.symbol {
            Symbol::Operand(_) => output.push(parsed_symbol),
            Symbol::Operator(op) => {
                let precedence = op.binary_precedence().ok_or(anyhow!(
                    "Illegal use of operator \"{}\" as a binary operator at {}-{}",
                    op,
                    parsed_symbol.start,
                    parsed_symbol.end
                ))?;

                while let Some(&top) = operator_stack.last() {
                    if top
                        .operator
                        .binary_precedence()
                        .expect("Illegal operator on shunting yard stack")
                        >= precedence
                    {
                        output.push(ParsedSymbol {
                            start: top.start,
                            end: top.end,
                            symbol: Symbol::Operator(top.operator),
                        });
                        operator_stack.pop();
                    } else {
                        break;
                    }
                }
                operator_stack.push(ParsedOperator {
                    operator: op,
                    start: parsed_symbol.start,
                    end: parsed_symbol.end,
                });
            }
        }
    }
    for operator in operator_stack.iter().rev() {
        output.push(ParsedSymbol {
            start: operator.start,
            end: operator.end,
            symbol: Symbol::Operator(operator.operator),
        });
    }

    Ok(output)
}

/// Removes all `None` values from a vector of `Option`s
fn to_cleaned_symbols<S>(symbols: &mut Vec<Option<S>>) -> () {
    symbols.retain(|s| s.is_some());
}

fn compose_expression(operator: ParsedOperator, lhs: ExprTree, rhs: ExprTree) -> Result<ExprTree> {
    let e: Expr<ExprTree> = match operator.operator {
        Operator::Asterisk => ExprMultiplication::from((lhs, rhs)).into(),
        Operator::Slash => ExprDivision::from((lhs, rhs)).into(),
        Operator::DoubleSlash => ExprIntDivision::from((lhs, rhs)).into(),
        Operator::Minus => ExprSubtraction::from((lhs, rhs)).into(),
        Operator::Plus => ExprAddition::from((lhs, rhs)).into(),
        Operator::Percent => ExprModulo::from((lhs, rhs)).into(),
        Operator::Caret => ExprPower::from((lhs, rhs)).into(),
        Operator::DoubleAsterisk => ExprPower::from((lhs, rhs)).into(),
        _ => {
            return Err(anyhow!(
                "Illegal use of \"{}\" as binary operator at {}-{}",
                operator.operator,
                operator.start,
                operator.end
            ))
        }
    };
    Ok(e.into())
}

fn wrap(expr: ExprTree, unary_operator: ParsedOperator) -> Result<ExprTree> {
    match unary_operator.operator {
        Operator::Plus => Ok(expr),
        Operator::Minus => {
            let expr_obj = ExprNegation::from(expr);
            Ok(ExprTree::from(Expr::Negation(expr_obj)))
        }
        _ => {
            return Err(anyhow!(
                "Illegal use of \"{}\" as unary operator at {}-{}",
                unary_operator.operator,
                unary_operator.start,
                unary_operator.end
            ))
        }
    }
}

#[derive(Debug)]
enum ExpressionInProgress {
    Operator {
        operator: ParsedOperator,
    },
    RHS {
        operator: ParsedOperator,
        rhs: ExprTree,
    },
    LHS {
        operator: ParsedOperator,
        lhs: ExprTree,
        rhs: ExprTree,
    },
}

impl ExpressionInProgress {
    /// Adds an operand to the expression in progress
    ///
    /// # Panics
    /// Panics if the expression is already complete
    pub(crate) fn add_operand(&mut self, operand: ExprTree) -> () {
        *self = match self {
            ExpressionInProgress::Operator { operator } => ExpressionInProgress::RHS {
                operator: *operator,
                rhs: operand,
            },
            ExpressionInProgress::RHS { operator, rhs } => ExpressionInProgress::LHS {
                operator: *operator,
                lhs: operand,
                rhs: rhs.clone(),
            },
            _ => panic!("Attempted to add operand to completed expression"),
        }
    }

    /// Composes the expression in progress into an expression tree
    ///
    /// # Panics
    /// Panics if the expression is incomplete
    pub(crate) fn compose_into(self) -> Result<ExprTree> {
        match self {
            ExpressionInProgress::Operator { operator: _ } => {
                panic!("Attempted to compose incomplete expression")
            }
            ExpressionInProgress::RHS {
                operator: _,
                rhs: _,
            } => {
                panic!("Attempted to compose incomplete expression")
            }
            ExpressionInProgress::LHS { operator, lhs, rhs } => {
                compose_expression(operator, lhs, rhs)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Expecting {
    Operator,
    Operand,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ParsedOperator {
    operator: Operator,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct ParsedSymbol {
    symbol: Symbol,
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub(crate) enum Symbol {
    Operand(ExprTree),
    Operator(Operator),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
    DoubleSlash,
    DoubleAsterisk,
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

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Asterisk => "*",
            Operator::Slash => "/",
            Operator::DoubleSlash => "//",
            Operator::DoubleAsterisk => "**",
            Operator::Backslash => "\\",
            Operator::Percent => "%",
            Operator::Caret => "^",
            Operator::Comma => ",",
            Operator::Equals => "==",
            Operator::NotEquals => "!=",
            Operator::LessThan => "<",
            Operator::LessThanOrEqual => "<=",
            Operator::GreaterThan => ">",
            Operator::GreaterThanOrEqual => ">=",
            Operator::Period => ".",
        };
        write!(f, "{}", s)
    }
}

impl Operator {
    pub(crate) fn binary_precedence(&self) -> Option<usize> {
        match self {
            Operator::Plus | Operator::Minus => Some(1),
            Operator::Asterisk | Operator::Slash | Operator::DoubleSlash | Operator::Percent => {
                Some(2)
            }
            Operator::DoubleAsterisk | Operator::Caret => Some(3),

            _ => None,
        }
    }
}
