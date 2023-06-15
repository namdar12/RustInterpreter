use crate::token::{Token, TokenType};
use std::{char, ops::IndexMut, str::Bytes,str};

use crate::error::{Result, Error};

//pub use error::{Error, Result};

pub struct Lexer{
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}


impl Lexer {
    pub fn new(input: String) -> Lexer{

        let lex = Lexer { 
             input: input.clone().into_bytes(),
             position: 0, 
             read_position: 0, 
             ch: 0 
            };
        lex
    }

    pub fn read_char(&mut self){
        if self.read_position >= self.input.len(){
            self.ch =0
        }else{
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position +=1
    }

    pub fn is_letter(& self) -> bool{
        self.ch.is_ascii_alphabetic() || self.ch == b'_'
    }

    pub fn read_identifier(&mut self) -> String {
        let positon = self.position;
        while (self.ch.is_ascii_alphabetic() || self.ch == b'_'){
            self.read_char()
        };
        return String::from_utf8_lossy(&self.input[positon..self.position]).to_string();
    }

    pub fn skip_whitespace(&mut self){
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn read_int(&mut self)-> String{
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }


        
    pub fn next_token(&mut self) -> Result<Token> {

        self.skip_whitespace();
        //self.skip_whitespace();
        let tok = match self.ch {
            b'{' => Token{Type: TokenType::LSquirly, Literal: self.ch.to_string()} ,
            b'}' => Token{Type: TokenType::RSquirly, Literal: self.ch.to_string()} ,
            b'(' => Token{Type: TokenType::Lparen, Literal: self.ch.to_string()} ,
            b')' => Token{Type: TokenType::Rparen, Literal: self.ch.to_string()} ,
            b'<' => Token{Type: TokenType::LessThan, Literal: self.ch.to_string()} ,
            b'>' => Token{Type: TokenType::GreaterThan, Literal: self.ch.to_string()} ,
            b'=' =>  Token{Type: TokenType::Equal, Literal: self.ch.to_string()} ,
            b',' => Token{Type: TokenType::Comma, Literal: self.ch.to_string()} ,
            b';' => Token{Type: TokenType::Semicolon, Literal: self.ch.to_string()} ,
            b'+' => Token{Type: TokenType::Plus, Literal: self.ch.to_string()} ,
            b'-' => Token{Type: TokenType::Minus, Literal: self.ch.to_string()} ,
            b'*' => Token{Type: TokenType::Asterisk, Literal: self.ch.to_string()} ,
            b'/' => Token{Type: TokenType::ForwardSlash, Literal: self.ch.to_string()} ,
            0 =>  Token{Type: TokenType::Eof, Literal: self.ch.to_string()},
            b'0'..=b'9' => Token{ Type: TokenType::Int, Literal: self.read_int()},
            _ => {if(self.ch.is_ascii_alphabetic() || self.ch == b'_') {
                    Token{Type: TokenType::Ident, Literal: self.read_identifier()}
                }else{
                    Token{Type: TokenType::Illegal, Literal: self.ch.to_string()}

                }
            }
        };

        Ok(tok)
    }
    


}


mod test {
    use super::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let expected = vec![
            Token {
                Type: TokenType::Assign,
                Literal: "=".to_string(),
            },
            Token {
                Type: TokenType::Plus,
                Literal: "+".to_string(),
            },
            Token {
                Type: TokenType::Lparen,
                Literal: "(".to_string(),
            },
            Token {
                Type: TokenType::Rparen,
                Literal: ")".to_string(),
            },
            Token {
                Type: TokenType::LSquirly,
                Literal: "{".to_string(),
            },
            Token {
                Type: TokenType::RSquirly,
                Literal: "}".to_string(),
            },
            Token {
                Type: TokenType::Comma,
                Literal: ",".to_string(),
            },
            Token {
                Type: TokenType::Semicolon,
                Literal: ";".to_string(),
            },
            Token {
                Type: TokenType::Eof,
                Literal: "".to_string(),
            },
        ];

        // let mut l = new(input);
        let mut l = Lexer::new(input);

        //ask about this
        for (i, tt) in expected.iter().enumerate() {

            let tok = l.next_token().unwrap();

            assert_eq!(
                tok.Type, tt.Type,
                "Test[{}] - token Type wrong. expected={:?}, got={:?}",
                i, tt.Type, tok.Type
            );

            assert_eq!(
                tok.Literal, tt.Literal,
                "Test[{}] - literal wrong. expected={}, got={}",
                i, tt.Literal, tok.Literal
            );
            
      
        }
    }

    #[test]
    fn variable_test(){
        //finish this test
        let input = "let five = 5 ;
        
        let ten = 10 ; 
        
        let add = fn (x, y) 
        { x + y;
        
        } ;";
        let expected = vec![
        Token {
            Type: TokenType::Let,
            Literal: "let".to_owned(),
        },
        Token {
            Type: TokenType::Ident,
            Literal: "five".to_string(),
        },
        Token {
            Type: TokenType::Assign,
            Literal: "=".to_string(),
        },
        Token {
            Type: TokenType::Int,
            Literal: "5".to_string(),
        }, 
        Token {
            Type: TokenType::Semicolon,
            Literal: ";".to_string(),
        },
        Token {
            Type: TokenType::Let,
            Literal: "let".to_string(),
        },
        Token {
            Type: TokenType::Ident,
            Literal: "ten".to_string(),
        },
        Token {
            Type: TokenType::Assign,
            Literal: "=".to_string(),
        },
        Token {
            Type: TokenType::Int,
            Literal: "10".to_string(),
        },
        Token {
            Type: TokenType::Semicolon,
            Literal: ";".to_string(),
        },
        Token {
            Type: TokenType::Let,
            Literal: "let".to_string(),
        },
        Token {
            Type: TokenType::Ident,
            Literal: "add".to_string(),
        },
        Token {
            Type: TokenType::Assign,
            Literal: "=".to_string(),
        },
        Token {
            Type: TokenType::Function,
            Literal: "fn".to_string(),
        },
        Token {
            Type: TokenType::Lparen,
            Literal: "(".to_string(),
        },
        Token {
            Type: TokenType::Ident,
            Literal: "x".to_string(),
        },
        Token {
            Type: TokenType::Comma,
            Literal: ",".to_string(),
        },
        Token {
            Type: TokenType::Ident,
            Literal: "y".to_string(),
        },
        Token {
            Type: TokenType::Rparen,
            Literal: ")".to_string(),
        },
        Token {
            Type: TokenType::LSquirly,
            Literal: "{".to_string(),
        },
        Token {
            Type: TokenType::Ident,
            Literal: "x".to_string(),
        },
        Token {
            Type: TokenType::Plus,
            Literal: "+".to_string(),
        },
        Token {
            Type: TokenType::Ident,
            Literal: "y".to_string(),
        },
        Token {
            Type: TokenType::Semicolon,
            Literal: ";".to_string(),
        },
        Token {
            Type: TokenType::RSquirly,
            Literal: "}".to_string(),
        },
        Token {
            Type: TokenType::Semicolon,
            Literal: ";".to_string(),
        }];

        let mut l = Lexer::new(input.to_owned());

        for (i, tt) in expected.iter().enumerate() {

            let mut tok = l.next_token().unwrap();

            println!("{tok:?}");

            assert_eq!(
                tok.Type, tt.Type,
                "Test[{}] - token Type wrong. expected={:?}, got={:?}",
                i, tt.Type, tok.Type
            );


            assert_eq!(
                tok.Literal, tt.Literal,
                "Test[{}] - literal wrong. expected={}, got={}",
                i, tt.Literal, tok.Literal
            );
            
        }




    }


    // fn test_peeker(){
    //     let input = " 10 == 10; 10 != 9";

    //     let mut l = Lexer::new(input.to_owned());

    //     //ask about this
    //     for (i, tt) in expected.iter().enumerate() {

    //         let tok = l.next_token();

    //         assert_eq!(
    //             tok.Type, tt.Type,
    //             "Test[{}] - token Type wrong. expected={}, got={}",
    //             i, tt.Type, tok.Type
    //         );

    //         assert_eq!(
    //             tok.Literal, tt.Literal,
    //             "Test[{}] - literal wrong. expected={}, got={}",
    //             i, tt.Literal, tok.Literal
    //         );
            
      
    //     }

    //}

}