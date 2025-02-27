use anyhow::{Ok, Result};
use into_expr_tree::into_expr_tree;

mod arguments;
mod cl_tools;
mod into_expr_tree;
mod parser;

fn main() -> Result<()> {
    println!("Enter expression:");
    let input = cl_tools::read_line();
    let input_chars = input.chars();
    let mut tokens = parser::Tokens::new(input_chars);

    let mut variable_names: Vec<String> = Vec::new();
    let expr_tree = into_expr_tree(&mut tokens, &mut variable_names, false)?;
    println!("Expression tree: {:?}", expr_tree);
    Ok(())
}
