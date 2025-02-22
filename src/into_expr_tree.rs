use anyhow::Result;
use fef::v0::expr::ExprTree;

use crate::parser::Token;

pub(crate) fn into_expr_tree(tokens: impl Iterator<Item = Result<Token>>) -> Result<ExprTree> {}
