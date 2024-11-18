use super::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    next_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            next_position: 1,
            current_char: None,
        };

        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.position);
        }
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.next_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.next_position - 1)
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char {
            Some(_) => {
                let literal = self.read_word();
                Token {
                    token_type: TokenType::Word,
                    literal,
                }
            }
            None => Token {
                token_type: TokenType::EOF,
                literal: String::new(),
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self) -> String {
        let start_position = self.position - 1;
        while let Some(c) = self.current_char {
            if c == ' ' || c == '\n' || c == '\t' {
                break;
            }
            self.read_char();
        }
        self.input[start_position..self.position - 1].to_string()
    }
}
