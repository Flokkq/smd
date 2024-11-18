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

                let literal = "#".repeat(level);
                if level > 6 {
                    Token {
                        token_type: TokenType::String,
                        literal,
                    }
                } else {
                    Token {
                        token_type: TokenType::Header,
                        literal,
                    }
                }
            }
            Some(_) => {
                let literal = self.read_word();
                Token {
                    token_type: TokenType::String,
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
            if c.is_whitespace() {
                break;
            }
            self.read_char();
        }
        self.input[start_position..self.position - 1].to_string()
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
    fn test_lexer_header() {
        let input = String::from("#\n######\n#######");

        let tests = vec![
            TestCase {
                expected_type: TokenType::Header,
                expected_literal: "#",
            },
            TestCase {
                expected_type: TokenType::Header,
                expected_literal: "######",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "#######",
            },
        ];

        let mut lexer = Lexer::new(input);

        for (i, test) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(
                tok.token_type, test.expected_type,
                "tests[{}] - tokentype wrong",
                i
            );
            assert_eq!(
                tok.literal, test.expected_literal,
                "tests[{}] - literal wrong",
                i
            );
        }
    }

    #[test]
    fn test_lexer_word() {
        let input =
            String::from("A space seperates a word.\n1234 Numbers_and_underscores are\talso cool and work t2g3th_er");

        let tests = vec![
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "A",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "space",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "seperates",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "a",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "word.",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "1234",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Numbers_and_underscores",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "are",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "also",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "cool",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "and",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "work",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "t2g3th_er",
            },
            TestCase {
                expected_type: TokenType::EOF,
                expected_literal: "",
            },
        ];

        let mut lexer = Lexer::new(input);

        for (i, test) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(
                tok.token_type, test.expected_type,
                "tests[{}] - tokentype wrong",
                i
            );
            assert_eq!(
                tok.literal, test.expected_literal,
                "tests[{}] - literal wrong",
                i
            );
        }
    }
}
