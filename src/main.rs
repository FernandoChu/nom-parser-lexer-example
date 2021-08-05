pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use lexer::lexer;
use parser::expression;
use token::{Token, Tokens};

fn main() {
    let (_, tokens) = lexer("Asdf + ASF").unwrap();
    println!("{:?}", expression(Tokens::new(&tokens)));
}
