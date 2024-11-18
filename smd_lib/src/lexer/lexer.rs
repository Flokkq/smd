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
            Some('#') => {
                let mut level = 0;
                while let Some('#') = self.current_char {
                    level += 1;
                    self.read_char();
                }

                if level > 6 {
                    let literal = self.read_text();
                    Token {
                        token_type: TokenType::Text,
                        literal: format!("#{}{}", "#".repeat(level - 1), literal),
                    }
                } else {
                    self.skip_whitespace();
                    let literal = self.read_text().to_string();
                    Token {
                        token_type: TokenType::Header(level),
                        literal,
                    }
                }
            }
            Some(_) => {
                let literal = self.read_text();
                Token {
                    token_type: TokenType::Text,
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

    fn read_text(&mut self) -> String {
        let start_position = self.position - 1;
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            self.read_char();
        }
        let text = self.input[start_position..self.position - 1].to_string();

        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::TokenType;

    struct TestCase<'a> {
        expected_type: TokenType,
        expected_literal: &'a str,
    }

    #[test]
    fn test_lexer_md_headers() {
        let input = String::from("# Header 1\n## Header 2\n### Header 3\n####### Invalid Header\n");

        let tests = vec![
            TestCase {
                expected_type: TokenType::Header(1),
                expected_literal: "Header 1",
            },
            TestCase {
                expected_type: TokenType::Header(2),
                expected_literal: "Header 2",
            },
            TestCase {
                expected_type: TokenType::Header(3),
                expected_literal: "Header 3",
            },
            TestCase {
                expected_type: TokenType::Text,
                expected_literal: "####### Invalid Header",
            },
            TestCase {
                expected_type: TokenType::EOF,
                expected_literal: "",
            },
        ];

        let mut lexer = Lexer::new(input);

        for (i, test) in tests.iter().enumerate() {
            let tok = lexer.next_token();

            if tok.token_type != test.expected_type {
                panic!(
                    "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                    i, test.expected_type, tok.token_type
                );
            }

            if tok.literal != test.expected_literal {
                panic!(
                    "tests[{}] - literal wrong. expected={}, got={}",
                    i, test.expected_literal, tok.literal
                );
            }
        }
    }
}
