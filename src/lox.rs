use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::process::exit;
use crate::scanner::Scanner;

pub(crate) struct Lox {
}

static mut HAD_ERROR: bool = false;

impl Lox {
    pub(crate) fn new() -> Self {
        Lox {}
    }

    pub(crate) fn run(&self, input: String) {
        let mut scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens();
        
        for t in tokens {
            println!("{:?}", t);
        }
    }
    
    pub(crate) fn run_file(&self, file_name: &String) {
        let mut file = File::open(file_name).unwrap();
        let mut s = String::new();
        let _ = file.read_to_string(&mut s);
        self.run(s);
    }
    
    pub(crate) fn run_prompt(&mut self) {
        loop {
            if unsafe { HAD_ERROR } {
                exit(65);
            }
            let mut buffer = String::new();
            unsafe { HAD_ERROR = false };
            print!("> ");
            let stdin = stdin();
            let _ = stdin.read_line(&mut buffer);
            self.run(buffer);
        }
    }

    pub(crate) fn error(line: u32, m: &str) {
        eprint!("[line {}] Error {}", line, m);
        unsafe { HAD_ERROR = true };
    }
}
