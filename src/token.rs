use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

lazy_static! {
    static ref TOKEN_LITERAL_MAP: HashMap<String, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("let".to_owned(), TokenKind::Let);
        map.insert("fn".to_owned(), TokenKind::Function);

        map
    };
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum TokenKind {
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self) // or provide a custom representation
    }
}

impl From<&str> for TokenKind {
    fn from(value: &str) -> Self {
        TOKEN_LITERAL_MAP
            .get(value)
            .map(|kind| kind.to_owned())
            .unwrap_or(Self::Ident)
    }
}

fn temp() {
    let kind_1 = TokenKind::from("let");
    let kind_2 = TokenKind::from("hello");
}
