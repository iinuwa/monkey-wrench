mod tests;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    bytes: &'a [u8],
    current_position: usize,
    next_position: usize,
    current_byte: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let mut lexer = Lexer {
            bytes: bytes,
            current_position: 0,
            next_position: 0,
            current_byte: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.next_position >= self.bytes.len() {
            self.current_byte = 0;
        } else {
            self.current_byte = self.bytes[self.next_position];
        }
        self.current_position = self.next_position;
        self.next_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_byte {
            b'=' => self.check_two_character_token(b'=', Token::Equal, Token::Assign),
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'!' => self.check_two_character_token(b'=', Token::NotEqual, Token::Bang),
            b'<' => Token::Less,
            b'>' => Token::Greater,
            0 => Token::EOF,
            x => {
                if Token::is_valid_identifier_char(x as char) {
                    return self.read_identifier();
                } else if x.is_ascii_digit() {
                    return self.read_integer();
                } else {
                    Token::Illegal(x.to_string())
                }
            }
        };
        self.read_char();
        token
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.current_position <= self.bytes.len() {
            tokens.push(self.next_token());
        }
        tokens
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.current_position;
        while Token::is_valid_identifier_char(self.current_byte as char) {
            self.read_char();
        }
        let token_bytes = &self.bytes[position..self.current_position];
        let identifier = String::from_utf8(token_bytes.to_vec()).unwrap();
        Token::lookup_identifier(identifier)
    }

    fn read_integer(&mut self) -> Token {
        let position = self.current_position;
        while self.current_byte.is_ascii_digit() {
            self.read_char();
        }
        let token_bytes = &self.bytes[position..self.current_position];
        let mut number: usize = 0;
        for (i, byte) in token_bytes.iter().rev().enumerate() {
            let digit = (*byte as char).to_digit(10).unwrap();
            number += (digit * 10u32.pow(i as u32)) as usize;
        }
        Token::Integer(number)
    }

    fn skip_whitespace(&mut self) {
        while self.current_byte.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> u8 {
        if self.current_position >= self.bytes.len() {
            return 0;
        }
        self.bytes[self.next_position]
    }

    fn check_two_character_token(
        &mut self,
        comparator: u8,
        true_token: Token,
        false_token: Token,
    ) -> Token {
        if self.peek_char() == comparator {
            self.read_char();
            true_token
        } else {
            false_token
        }
    }
}
