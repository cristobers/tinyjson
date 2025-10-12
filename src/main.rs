mod lexer;
mod parser;
mod token;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::TokenKind;

use std::env;
use std::fs;
use std::io;

use std::process;

fn main() {
    let args: String = env::args().nth(1).expect("Failed to get input JSON");

    let json_file: String = fs::read_to_string(args).unwrap();

    let source: String = String::from(json_file);
    let mut lexer: Lexer = Lexer::new(source);
    let mut parser: Parser = Parser::new(&mut lexer);

    parser.parse();
}
