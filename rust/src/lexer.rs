use std::str::Chars;

pub struct Lexer<'a> {
    source: Chars<'a>,
    pub tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str) -> Lexer<'a> {
        Lexer {
            source: string.chars(),
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Result<(), &'static str> {
        for char in self.source.clone() {
            Token::match_char(char).map(|token| self.tokens.push(token));
        }
        Ok(())
    }
}

pub enum Token {
    GreaterThan,
    SmallerThan,
    Plus,
    Minus,
    Point,
    Comma,
    BracketOpen,
    BracketClose,
}

impl Token {
    pub fn match_char(c: char) -> Option<Token> {
        match c {
            '>' => Some(Token::GreaterThan),
            '<' => Some(Token::SmallerThan),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '.' => Some(Token::Point),
            ',' => Some(Token::Comma),
            '[' => Some(Token::BracketOpen),
            ']' => Some(Token::BracketClose),
            _ => None,
        }
    }
}
