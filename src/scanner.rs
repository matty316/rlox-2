use std::any::Any;
use std::collections::HashMap;

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

        self.tokens.push(Token::new(EOF, "", self.line));
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
                } else if Self::is_alpha(c){
                    self.ident();
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
        let text = &self.input[self.start..self.current];
        let t = Token::new(t, text, self.line);
        self.tokens.push(t);
    }

    fn add_token(&mut self, t: TokenType, literal: impl Any) {
        let text = &self.input[self.start..self.current];
        let t = Token::new_literal(t, text, literal, self.line);
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

    fn ident(&mut self) {
        while Self::is_alphanumeric(self.peek()) { self.advance(); }

        let keywords = Self::keywords();

        let t = &self.input[self.start..self.current].to_string();
        
        match keywords.get(t) {
            Some(t) => self.add_empty_token(*t),
            None => self.add_empty_token(IDENT),
        }
    }

    fn is_digit(c: u8) -> bool {
        b'0' <= c && c <= b'9'
    }

    fn is_alpha(c: u8) -> bool {
        c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z' || c == b'_'
    }

    fn is_alphanumeric(c: u8) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn keywords() -> HashMap<String, TokenType> {
        HashMap::from([
            ("and".to_string(), AND),
            ("class".to_string(), CLASS),
            ("else".to_string(), ELSE),
            ("false".to_string(), FALSE),
            ("for".to_string(), FOR),
            ("fun".to_string(), FUN),
            ("if".to_string(), IF),
            ("nil".to_string(), NIL),
            ("or".to_string(), OR),
            ("print".to_string(), PRINT),
            ("return".to_string(), RETURN),
            ("super".to_string(), SUPER),
            ("this".to_string(), THIS),
            ("true".to_string(), TRUE),
            ("var".to_string(), VAR),
            ("while".to_string(), WHILE),
        ])
    }
}

mod tests {
    use std::{any::TypeId, f32::consts::E};

    use super::*;
        
    #[test]
    fn test_scan_single_char_and_double_tokens() {
        let input = "(){},.-+;*
        < > = /
        <= >= == //
        ";

        let exp = vec![
            Token::new(LPAREN, "(", 1), 
            Token::new(RPAREN, ")", 1), 
            Token::new(LBRACE, "{", 1), 
            Token::new(RBRACE, "}", 1), 
            Token::new(COMMA, ",", 1), 
            Token::new(DOT, ".", 1), 
            Token::new(MINUS, "-", 1), 
            Token::new(PLUS, "+", 1),
            Token::new(SEMICOLON, ";", 1),
            Token::new(STAR, "*", 1), 
            Token::new(LT, "<", 2), 
            Token::new(GT, ">", 2), 
            Token::new(EQ, "=", 2), 
            Token::new(SLASH, "/", 2), 
            Token::new(LTEQ, "<=", 3), 
            Token::new(GTEQ, ">=", 3), 
            Token::new(EQEQ, "==", 3), 
            Token::new(EOF, "", 3)
        ];

        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();
        for (i, e) in exp.into_iter().enumerate() {
            let t = &tokens[i];
            assert_eq!(e.token_type, t.token_type);
            assert_eq!(e.lexeme, t.lexeme);
            assert_eq!(e.line, t.line)
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
            Token::new(BANG, "!", 2),
            Token::new(LT, "<", 3),
            Token::new(GT, ">", 4),
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
            "this is a string"
        "#;
        
        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();
        let t = &tokens[0];
        assert_eq!(t.token_type, STRING);
        assert_eq!(t.lexeme, "\"this is a string\"");
        let s: &String = t.literal.downcast_ref().unwrap();
        assert_eq!(s, &"this is a string".to_string());
    } 

    #[test]
    fn test_numbers() {
        let input = "1
        34 69 420
        6.9
        42.0
        ";

        let exp = vec![
            Token::new_literal(NUM, "1", 1.0, 1),
            Token::new_literal(NUM, "34", 34.0, 2),
            Token::new_literal(NUM, "69", 69.0, 2),
            Token::new_literal(NUM, "420", 420.0, 2),
            Token::new_literal(NUM, "6.9", 6.9, 3),
            Token::new_literal(NUM, "42.0", 42.0, 4),
        ];

        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();

        for (i, e) in exp.into_iter().enumerate() {
            let t = &tokens[i];
            assert_eq!(e.token_type, NUM);
            assert_eq!(e.lexeme, t.lexeme);
            assert_eq!(e.line, t.line);
            let n: &f64 = t.literal.downcast_ref().unwrap();
            let en: &f64 = e.literal.downcast_ref().unwrap();
            assert_eq!(n, en);
        }
    }

    #[test]
    fn test_idents() {
        let input = "
        num num1
        ";

        let exp = vec![
            Token::new(IDENT, "num", 2),
            Token::new(IDENT, "num1", 2),
        ];

        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();

        for (i, e) in exp.into_iter().enumerate() {
            let t = &tokens[i];
            assert_eq!(e.token_type, IDENT);
            assert_eq!(e.lexeme, t.lexeme);
            assert_eq!(e.line, t.line);
            let ident: &String = t.literal.downcast_ref().unwrap();
            let eident: &String = e.literal.downcast_ref().unwrap();
            assert_eq!(eident, ident);
        }
    }

    #[test]
    fn test_keywords() {
        let input = "and class else false for fun if nil or print return super this true var while";
        
        let exp = vec![
            Token::new(AND, "and", 1),
            Token::new(CLASS, "class", 1),
            Token::new(ELSE, "else", 1),
            Token::new(FALSE, "false", 1),
            Token::new(FOR, "for", 1),
            Token::new(FUN, "fun", 1),
            Token::new(IF, "if", 1),
            Token::new(NIL, "nil", 1),
            Token::new(OR, "or", 1),
            Token::new(PRINT, "print", 1),
            Token::new(RETURN, "return", 1),
            Token::new(SUPER, "super", 1),
            Token::new(THIS, "this", 1),
            Token::new(TRUE, "true", 1),
            Token::new(VAR, "var", 1),
            Token::new(WHILE, "while", 1),
            Token::new(EOF, "", 1),
        ];

        let mut s = Scanner::new(input.to_string());
        let tokens = s.scan_tokens();

        for (i, e) in exp.into_iter().enumerate() {
            let t = &tokens[i];
            assert_eq!(e.token_type, t.token_type);
            assert_eq!(e.lexeme, t.lexeme);
            assert_eq!(e.line, t.line);
        }
    }
}
