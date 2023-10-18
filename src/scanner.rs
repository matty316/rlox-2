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
            b'"' => self.string(),
            b'\n' => self.line += 1,
            b' ' | b'\t' | b'\r' => (),
            _ => {
                if Self::is_digit(c) {
                    self.number();
                } else {
                    Lox::error(self.line, "Unexpected char")
                }
            }
        }
    }

    //Helpers
    fn peek(&self) -> u8 {
        if self.is_at_end() { return b'\0'; }

        return self.input.as_bytes()[self.current];
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.input.len() { return b'\0'; }

        return self.input.as_bytes()[self.current + 1];
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

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let s = &self.input[self.start+1..self.current-1];
        self.add_token(STRING, s.to_string())
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) { self.advance(); }

        if self.peek() == b'.'  && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) { self.advance(); }
        }

        let s = &self.input[self.start..self.current];
        let n: f64 = s.parse().unwrap();
        self.add_token(NUM, n)
    }

    fn is_digit(c: u8) -> bool {
        b'0' <= c && c <= b'9'
    }
}

mod tests {
    use std::any::TypeId;

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

    #[test]
    fn test_strings() {
        let input = r#"
            "string"
        "#;
        
        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();
        let t = &tokens[0];
        assert_eq!(t.token_type, STRING);
        assert_eq!(t.lexeme, "\"string\"");
        let s: &String = t.literal.downcast_ref().unwrap();
        assert_eq!(s, &"string".to_string());
    } 

    #[test]
    fn test_numbers() {
        let input = "1
        34
        69
        420
        6.9
        42.0
        ";

        let exp = vec![
            Token::new(NUM, "1", 1.0, 1),
            Token::new(NUM, "34", 34.0, 1),
            Token::new(NUM, "69", 69.0, 1),
            Token::new(NUM, "420", 420.0, 1),
            Token::new(NUM, "6.9", 6.9, 1),
            Token::new(NUM, "42.0", 42.0, 1),
        ];

        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();

        for (i, e) in exp.into_iter().enumerate() {
            let t = &tokens[i];
            assert_eq!(e.token_type, NUM);
            assert_eq!(e.lexeme, t.lexeme);
            let n: &f64 = t.literal.downcast_ref().unwrap();
            let en: &f64 = e.literal.downcast_ref().unwrap();
            assert_eq!(n, en);
        }
    }

    //Test Helpers
    fn create_token(t: TokenType, line: u32) -> Token {
        Token::new_empty(t, line)
    }
}
