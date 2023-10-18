use std::any::Any;

use crate::token::TokenType;
use crate::token::TokenType::*;
use crate::token::Token;
use crate::lox::Lox;

pub(crate) struct Scanner {
    input: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub(crate) fn new(input: String) -> Self {
        Scanner { 
            input: input,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1, 
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }   

        self.add_empty_token(EOF);
        return &self.tokens;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_empty_token(LPAREN),
            b')' => self.add_empty_token(RPAREN),
            b'{' => self.add_empty_token(LBRACE),
            b'}' => self.add_empty_token(RBRACE),
            b',' => self.add_empty_token(COMMA),
            b'.' => self.add_empty_token(DOT),
            b'-' => self.add_empty_token(MINUS),
            b'+' => self.add_empty_token(PLUS),
            b';' => self.add_empty_token(SEMICOLON),
            b'*' => self.add_empty_token(STAR),
            b'!' => {
                if self.match_two_char(b'=') {
                    self.add_empty_token(BANGEQ);
                } else {
                    self.add_empty_token(BANG);
                }
            }
            b'=' => {
                if self.match_two_char(b'=') {
                    self.add_empty_token(EQEQ);
                } else {
                    self.add_empty_token(EQ);
                }
            }
            b'<' => {
                if self.match_two_char(b'=') {
                    self.add_empty_token(LTEQ);
                } else {
                    self.add_empty_token(LT);
                }
            }
            b'>' => {
                if self.match_two_char(b'=') {
                    self.add_empty_token(GTEQ);
                } else {
                    self.add_empty_token(GT);
                }
            }
            b'/' => {
                if self.match_two_char(b'/') {
                    while self.peek() == b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_empty_token(SLASH);
                }
            }
            b'\n' => self.line += 1,
            b' ' | b'\t' | b'\r' => (),
            _ => Lox::error(self.line, ""),
        }
    }

    //Helpers
    fn peek(&self) -> u8 {
        if self.is_at_end() { return b'\0'; }

        return self.input.as_bytes()[self.current];
    }

    fn advance(&mut self) -> u8 {
        let current = self.current;
        self.current += 1;
        return self.input.as_bytes()[current];
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn add_empty_token(&mut self, t: TokenType) {
        let t = Token::new_empty(t, self.line);
        self.tokens.push(t);
    }

    fn add_token(&mut self, t: TokenType, literal: impl Any) {
        let text = &self.input[self.start..self.current];
        let t = Token::new(t, text, literal, self.line);
        self.tokens.push(t)
    }

    fn match_two_char(&mut self, c: u8) -> bool {
        if self.is_at_end() { return false; }
        if self.input.as_bytes()[self.current] != c { return false; }

        self.current += 1;
        return true;
    }
}

mod tests {
    use super::*;
        
    #[test]
    fn test_scan_single_char_and_double_tokens() {
        let input = "(){},.-+;*
        < > = /
        <= >= == //
        ";

        let exp = vec![
            LPAREN, RPAREN, LBRACE, RBRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON,STAR, LT, GT, EQ, SLASH, LTEQ, GTEQ, EQEQ, EOF
        ];

        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();
        for (i, e) in exp.into_iter().enumerate() {
            let t = create_token(e, 1);
            assert_eq!(t.token_type, tokens[i].token_type);
            assert_eq!(t.lexeme, "");
        }
    }

    #[test]
    fn test_line_numbers() {
        let input = "
        !
        <
        >
        ";

        let exp = vec![
            create_token(BANG, 2),
            create_token(LT, 3),
            create_token(GT, 4),
        ];

        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();
        for (i, e) in exp.into_iter().enumerate() {
            assert_eq!(e.line, tokens[i].line)
        }
    }

    //Test Helpers
    fn create_token(t: TokenType, line: u32) -> Token {
        Token::new_empty(t, line)
    }
}
