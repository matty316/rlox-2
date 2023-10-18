mod scanner;
mod lox;
mod token;

use std::env;
use crate::lox::Lox;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    if args.len() > 1 {
        println!("Usage: rlox [script]");
    } else if args.len() == 1 {
        lox.run_file(&args[0]);
    } else {
        lox.run_prompt();
    }
}
