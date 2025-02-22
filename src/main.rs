mod arguments;
mod cl_tools;
mod into_expr_tree;
mod parser;

fn main() {
    println!("Enter expression:");
    let input = cl_tools::read_line();
    let input_chars = input.chars();
    let tokens = parser::Tokens::new(input_chars);

    for token in tokens {
        match token {
            Ok(parsed_token) => println!("{:?}", parsed_token),
            Err(e) => eprintln!("{}", e),
        }
    }
}
