use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;


#[derive(Debug, PartialEq)]
pub enum TokenType {
    Ident,
    Int,

    Illegal,
    Eof,

    Lparen,
    Rparen,
    LSquirly,
    RSquirly,

    Assign,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Not,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
#[derive(Debug)]
pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}