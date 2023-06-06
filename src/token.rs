use std::fmt;
use std::collections::HashMap;


#[derive(PartialEq)]
#[derive(Debug)]
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
            ("fn", TokenKind::Function),
            ("let", TokenKind::Let),
        ];

        let hash = initial_values.into_iter().collect();
        return hash
        
    }

    pub fn look_up_ident(&self,ident: String) -> TokenKind {
        let keywords = self.get_keywords();

        match keywords.get(&ident) {
            Some(token_type) =>  1000, //token_type.clone(),  // assuming TokenType implements Clone
            None => 1000,
        };

        return TokenKind::Assign
    }
}

