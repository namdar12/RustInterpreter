use crate::token::{Token, TokenKind};
use std::char;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            input: input.clone(),
            position: 0,
            read_position: 0,
            ch: input.as_bytes()[0] as char,
        };
        lex.read_char();

        lex
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(){
            self.read_char()
        }
        self.input[position..self.position].to_owned()

    }

    pub fn skip_white_space(&mut self){
        println!("Skipped");
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn is_letter(&self) -> bool {
        return 'a' <= self.ch && self.ch <= 'z'
            || 'A' <= self.ch && self.ch <= 'Z'
            || self.ch == '_';
    }

    pub fn is_digit(&self) -> bool{
        self.ch >= '0' && self.ch <= '9'
    }


    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }

        self.input
            .chars()
            .skip(position)
            .take(self.position - position)
            .collect()
    }

    pub fn peek_char(&mut self) -> char{
        if self.read_position >= self.input.len(){
            return '\0';
        }else{
            return self.input.as_bytes()[self.read_position] as char;
        }


    }

    pub fn next_token(&mut self) -> Token {
        let mut tok = Token {
            kind: TokenKind::EOF,
            value: self.input.clone(),
        };

        self.skip_white_space();

        match self.ch {
            '(' => {
                tok.kind = TokenKind::Lparen;
                tok.value = self.ch.to_string()
            }
            ')' => {
                tok.kind = TokenKind::Rparen;
                tok.value = self.ch.to_string();
            }
            '{' => {
                tok.kind = TokenKind::Lbrace;
                tok.value = self.ch.to_string();
            }
            '}' => {
                tok.kind = TokenKind::Rbrace;
                tok.value = self.ch.to_string();
            }
            '+' => {
                tok.kind = TokenKind::Plus;
                tok.value = self.ch.to_string();
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok.value = "==".to_owned();
                    tok.kind = TokenKind::EQ;
                }else{
                    tok.kind = TokenKind::Assign;
                    tok.value = self.ch.to_string();
                } 
            }
            ';' => {
                tok.kind = TokenKind::Semicolon;
                tok.value = self.ch.to_string();
            }
            ',' => {
                tok.kind = TokenKind::Comma;
                tok.value = self.ch.to_string();
            }
            '!' =>{
                if self.peek_char() == '=' {
                    self.read_char();
                    tok.value = "!=".to_owned();
                    tok.kind = TokenKind::NotEq;
                }else{
                    tok.kind = TokenKind::Bang;
                    tok.value = self.ch.to_string();
                } 
            }
            '\0' => {
                tok.kind = TokenKind::EOF;
                tok.value = "".to_string();
            }
            _ => {
                if self.is_letter() {
                    {
                        tok.value = self.read_identifier();
                        tok.kind = TokenKind::from(tok.value.as_str());
                    }
                } else if self.is_digit() {
                    {
                        tok.value = self.read_number();
                        tok.kind = TokenKind::Int;
                    }
                }else {
                    {
                        tok.kind = TokenKind::Illegal;
                        tok.value = self.ch.to_string();
                    }
                }
            }
        }

        self.read_char();
        tok
    }
    
}

mod test {
    use super::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let expected = vec![
            Token {
                kind: TokenKind::Assign,
                value: "=".to_string(),
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
            },
        ];

        // let mut l = new(input);
        let mut l = Lexer::new(input);

        //ask about this
        for (i, tt) in expected.iter().enumerate() {

            let tok = l.next_token();

            assert_eq!(
                tok.kind, tt.kind,
                "Test[{}] - token kind wrong. expected={}, got={}",
                i, tt.kind, tok.kind
            );

            assert_eq!(
                tok.value, tt.value,
                "Test[{}] - literal wrong. expected={}, got={}",
                i, tt.value, tok.value
            );
            
      
        }
    }

    #[test]
    fn identifier_test() {
        let input = "hello";
        let expected = Token {
            kind: TokenKind::Ident,
            value: input.to_owned(),
        };

        let mut lexer = Lexer::new(input.to_owned());

        assert_eq!(lexer.next_token(), expected);
    }

    #[test]
    fn variable_test(){
        //finish this test
        let input = "let five = 5 ;
        
        let ten = 10 ; 
        
        let add = fn ( x , y ) 
        { x + y;
        
        } ;";
        let expected = vec![
        Token {
            kind: TokenKind::Let,
            value: "let".to_owned(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "five".to_string(),
        },
        Token {
            kind: TokenKind::Assign,
            value: "=".to_string(),
        },
        Token {
            kind: TokenKind::Int,
            value: "5".to_string(),
        }, 
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        },
        Token {
            kind: TokenKind::Let,
            value: "let".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "ten".to_string(),
        },
        Token {
            kind: TokenKind::Assign,
            value: "=".to_string(),
        },
        Token {
            kind: TokenKind::Int,
            value: "10".to_string(),
        },
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        },
        Token {
            kind: TokenKind::Let,
            value: "let".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "add".to_string(),
        },
        Token {
            kind: TokenKind::Assign,
            value: "=".to_string(),
        },
        Token {
            kind: TokenKind::Function,
            value: "fn".to_string(),
        },
        Token {
            kind: TokenKind::Lparen,
            value: "(".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "x".to_string(),
        },
        Token {
            kind: TokenKind::Comma,
            value: ",".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "y".to_string(),
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
            kind: TokenKind::Ident,
            value: "x".to_string(),
        },
        Token {
            kind: TokenKind::Plus,
            value: "+".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "y".to_string(),
        },
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        },
        Token {
            kind: TokenKind::Rbrace,
            value: "}".to_string(),
        },
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        }];

        let mut l = Lexer::new(input.to_owned());

        //ask about this
        for (i, tt) in expected.iter().enumerate() {

            let tok = l.next_token();

            println!("{tok:?}");

            assert_eq!(
                tok.kind, tt.kind,
                "Test[{}] - token kind wrong. expected={}, got={}",
                i, tt.kind, tok.kind
            );

            assert_eq!(
                tok.value, tt.value,
                "Test[{}] - literal wrong. expected={}, got={}",
                i, tt.value, tok.value
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
    //             tok.kind, tt.kind,
    //             "Test[{}] - token kind wrong. expected={}, got={}",
    //             i, tt.kind, tok.kind
    //         );

    //         assert_eq!(
    //             tok.value, tt.value,
    //             "Test[{}] - literal wrong. expected={}, got={}",
    //             i, tt.value, tok.value
    //         );
            
      
    //     }

    //}


}
mod test {
    use super::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let expected = vec![
            Token {
                kind: TokenKind::Assign,
                value: "=".to_string(),
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
            },
        ];

        // let mut l = new(input);
        let mut l = Lexer::new(input);

        //ask about this
        for (i, tt) in expected.iter().enumerate() {

            let tok = l.next_token();

            assert_eq!(
                tok.kind, tt.kind,
                "Test[{}] - token kind wrong. expected={}, got={}",
                i, tt.kind, tok.kind
            );

            assert_eq!(
                tok.value, tt.value,
                "Test[{}] - literal wrong. expected={}, got={}",
                i, tt.value, tok.value
            );
            
      
        }
    }

    #[test]
    fn identifier_test() {
        let input = "hello";
        let expected = Token {
            kind: TokenKind::Ident,
            value: input.to_owned(),
        };

        let mut lexer = Lexer::new(input.to_owned());

        assert_eq!(lexer.next_token(), expected);
    }

    #[test]
    fn variable_test(){
        //finish this test
        let input = "let five = 5 ;
        
        let ten = 10 ; 
        
        let add = fn ( x , y ) 
        { x + y;
        
        } ;";
        let expected = vec![
        Token {
            kind: TokenKind::Let,
            value: "let".to_owned(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "five".to_string(),
        },
        Token {
            kind: TokenKind::Assign,
            value: "=".to_string(),
        },
        Token {
            kind: TokenKind::Int,
            value: "5".to_string(),
        }, 
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        },
        Token {
            kind: TokenKind::Let,
            value: "let".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "ten".to_string(),
        },
        Token {
            kind: TokenKind::Assign,
            value: "=".to_string(),
        },
        Token {
            kind: TokenKind::Int,
            value: "10".to_string(),
        },
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        },
        Token {
            kind: TokenKind::Let,
            value: "let".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "add".to_string(),
        },
        Token {
            kind: TokenKind::Assign,
            value: "=".to_string(),
        },
        Token {
            kind: TokenKind::Function,
            value: "fn".to_string(),
        },
        Token {
            kind: TokenKind::Lparen,
            value: "(".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "x".to_string(),
        },
        Token {
            kind: TokenKind::Comma,
            value: ",".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "y".to_string(),
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
            kind: TokenKind::Ident,
            value: "x".to_string(),
        },
        Token {
            kind: TokenKind::Plus,
            value: "+".to_string(),
        },
        Token {
            kind: TokenKind::Ident,
            value: "y".to_string(),
        },
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        },
        Token {
            kind: TokenKind::Rbrace,
            value: "}".to_string(),
        },
        Token {
            kind: TokenKind::Semicolon,
            value: ";".to_string(),
        }];

        let mut l = Lexer::new(input.to_owned());

        //ask about this
        for (i, tt) in expected.iter().enumerate() {

            let tok = l.next_token();

            println!("{tok:?}");

            assert_eq!(
                tok.kind, tt.kind,
                "Test[{}] - token kind wrong. expected={}, got={}",
                i, tt.kind, tok.kind
            );

            assert_eq!(
                tok.value, tt.value,
                "Test[{}] - literal wrong. expected={}, got={}",
                i, tt.value, tok.value
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
    //             tok.kind, tt.kind,
    //             "Test[{}] - token kind wrong. expected={}, got={}",
    //             i, tt.kind, tok.kind
    //         );

    //         assert_eq!(
    //             tok.value, tt.value,
    //             "Test[{}] - literal wrong. expected={}, got={}",
    //             i, tt.value, tok.value
    //         );
            
      
    //     }

    //}


}