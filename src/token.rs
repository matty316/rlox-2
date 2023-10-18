use std::any::Any;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum TokenType {
    LPAREN, RPAREN, LBRACE, RBRACE, COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    BANG, BANGEQ, EQ, EQEQ, GT, LT, GTEQ, LTEQ, 

    IDENT, STRING, NUM,

    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR, PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF 
}

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Box<dyn Any>,
    pub(crate) line: u32,
}

impl Token {
    pub(crate) fn new_literal(t: TokenType, lexeme: &str, literal: impl Any, line: u32) -> Self {
        Token {
            token_type: t,
            lexeme: lexeme.to_string(),
            literal: Box::new(literal),
            line: line,
        }
    }

    pub(crate) fn new(t: TokenType, lexeme: &str, line: u32) -> Self {
        Token { token_type: t, lexeme: lexeme.to_string(), literal: Box::new("".to_string()), line: line }
    }
}