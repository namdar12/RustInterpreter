use std::{ops::IndexMut, str::Bytes,char};

use crate::token::{Token, TokenKind};

pub struct lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl lexer {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'; 
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0');
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        
        let mut tok = Token{
            kind: TokenKind::EOF,
            value: String::from(""),
        };

        match self.ch {

            '(' => {tok.kind = TokenKind::Lparen; tok.value = self.ch.to_string();},
            ')' => {tok.kind = TokenKind::Rparen; tok.value = self.ch.to_string();},
            '{' => {tok.kind = TokenKind::Lbrace; tok.value = self.ch.to_string();},
            '}' => {tok.kind = TokenKind::Rbrace; tok.value = self.ch.to_string();},
            '+' => {tok.kind = TokenKind::Plus; tok.value = self.ch.to_string();},
            '=' => {tok.kind = TokenKind::Assign; tok.value = self.ch.to_string();},
            ';' => {tok.kind = TokenKind::Semicolon; tok.value = self.ch.to_string();},
            ',' => {tok.kind = TokenKind::Comma; tok.value = self.ch.to_string();},


            _ => {tok.kind = TokenKind::EOF; tok.value = "".to_string() ;},


        }


        return tok
   
   
    }

}







pub fn test_next_token(){
    let input = String::from("=+(){},;");

    let expected = vec![
        Token { 
            kind: TokenKind::Assign, 
            value: "=".to_string() 
        },
        Token {
            kind: TokenKind::Plus,
            value: "+".to_string(),
        },
        Token {
            kind: TokenKind::Lparen,
            value: "(".to_string(),
        },
        Token {
            kind: TokenKind::Rparen,
            value: ")".to_string(),
        },
        Token {
            kind: TokenKind::Lbrace,
            value: "{".to_string(),
        },
        Token {
            kind: TokenKind::Rbrace,
            value: "}".to_string(),
        },
        Token {
            kind: TokenKind::Comma,
            value: ",".to_string(),
        },
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        },
        Token {
            kind: TokenKind::EOF,
            value: "".to_string(),
        } ];

    let mut l = new("test".to_string());

    //ask about this
    for (i,  tt) in expected.iter().enumerate() {

        let tok = l.next_token();

        assert_eq!(tok.kind, tt.kind, "Test[{}] - tokentype wrong. expected={}, got={}", i, tt.kind, tok.kind);
        assert_eq!(tok.value, tt.value, "Test[{}] - literal wrong. expected={}, got={}", i, tt.value, tok.value);

    }

    
}

pub fn new(input: String) -> lexer {
   let mut lex = lexer { input: "hello".to_string(), position:100, read_position: 10,ch: 'a'};
   lex.read_char();
   return lex
}



// pub fn add(x:u8, y:u8) -> u8 {
//     x + y
// }

// mod test {
//     use super::add;

//     #[test]
//     fn add_works() {
//         let (x, y) = (1,2 );
//         let expected = 3;
//         let output = add(x,y);

//         assert_eq!(expected, output);
//     }

//     #[test]
//     fn add_fails() {
//         add(200, 100);
//     }
// }