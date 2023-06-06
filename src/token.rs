use std::fmt;
use std::collections::HashMap;


#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]

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
    Let
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)  // or provide a custom representation
    }

}

impl TokenKind{
    pub fn get_keywords(&self) -> HashMap<String, TokenKind> {
        let initial_values= vec![
            ("fn".to_string(), TokenKind::Function),
            ("let".to_string(), TokenKind::Let),
        ];

        let hash = initial_values.into_iter().collect();
        return hash
        
    }

    pub fn look_up_ident(&self,ident: String) -> Option<TokenKind>{
        let keywords = self.get_keywords();
        match keywords.get(&ident) {
            Some(TokenKind) => Some(*TokenKind) ,  // assuming TokenType implements Clone
            None => None,
        }
    }
}

