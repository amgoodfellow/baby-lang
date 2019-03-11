use std::io::{self, Write, BufRead};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    //Keywords
    Let,
    Function,

    //Operators
    Plus,
    Minus,
    Assignment,
    Equality,
    LessThan,
    RightThan,
    LEQ,
    REQ,
    //Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    //Literals
    Symbol(String),
    Number(String),
    //Other
    EOF,
    Illegal
}

impl Default for Token {
    fn default() -> Token {
        Token::Illegal
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable()
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    pub fn read_number(&mut self, initial: char) -> String {
        let mut num = String::new();
        num.push(initial);
        while let Some(&c) = self.input.peek() {
            if c.is_numeric() {
                num.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        num
    }

    pub fn read_symbol(&mut self, initial: char) -> Token {
        let mut sym = String::new();
        sym.push(initial);
        while let Some(&c) = self.input.peek() {
            if c.is_alphabetic() {
                sym.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        match sym.as_ref() {
            "let" => Token::Let,
            "fn" => Token::Function,
            _ => Token::Symbol(sym)
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.input.next() {
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some(';') => Token::Semicolon,
            Some('=') => {
                if self.input.peek().unwrap().clone() == '=' {
                    self.input.next();
                    Token::Equality
                } else {
                    Token::Assignment
                }
            }
            Some(ch) => {
                if ch.is_numeric() {
                    Token::Number(self.read_number(ch))
                } else if ch.is_alphabetic() {
                    self.read_symbol(ch)
                }else {
                    Token::Illegal
                }
            }
            _ => Token::EOF
        }
    }
}

fn main() {
    let stdin = io::stdin();

    loop {
        print!("lispy-boi> ");
        io::stdout().flush().expect("Error flushing stdout");
        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let mut lexer = Lexer::new(&line);
        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == Token::EOF {
                break;
            }
        }
    }
}
