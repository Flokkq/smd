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
            Some('\n') => {
                self.read_char();
                Token {
                    token_type: TokenType::NewLine,
                    literal: "\n".to_string(),
                }
            }
            Some('*') => {
                let mut depth = 0;
                while let Some('*') = self.current_char {
                    depth += 1;
                    self.read_char();
                }

                let token_type = match depth {
                    3 => TokenType::BoldItalic,
                    2 => TokenType::Bold,
                    1 => TokenType::Italic,
                    _ => TokenType::ILLEGAL, // TODO: weird edge cases
                };

                Token {
                    token_type,
                    literal: "*".repeat(depth),
                }
            }
            Some('_') => {
                let mut depth = 0;
                while let Some('_') = self.current_char {
                    depth += 1;
                    self.read_char();
                }

                let token_type = match depth {
                    3 => TokenType::BoldItalic,
                    2 => TokenType::Bold,
                    1 => TokenType::Italic,
                    _ => TokenType::ILLEGAL, // TODO: weird edge cases
                };

                Token {
                    token_type,
                    literal: "_".repeat(depth),
                }
            }
            Some('~') => {
                let mut depth = 0;
                while let Some('~') = self.current_char {
                    depth += 1;
                    self.read_char();
                }

                let token_type = if depth == 2 {
                    TokenType::Strikethrough
                } else {
                    TokenType::ILLEGAL
                };

                Token {
                    token_type,
                    literal: "~".repeat(depth),
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
            if c == ' ' || c == '\t' {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self) -> String {
        let start_position = self.position - 1;
        while let Some(c) = self.current_char {
            if c.is_whitespace() || c == '*' || c == '_' || c == '~' {
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
    fn test_comprehensive() {
        let input = String::from(
            "# Header 1\n\
        Some text below header.\n\
        **Bold Text** _Italic Text_ ***Bold and Italic Text***\n\
        ~~Strikethrough~~\n\
        ## Subheader 1\n\
        - Bullet 1\n\
        - Bullet 2\n\n\
        Another paragraph here.\n\
        ###### Small Header\n\
        ####### Invalid Header\n\
        End of file.",
        );

        let tests = vec![
            TestCase {
                expected_type: TokenType::Header,
                expected_literal: "#",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Header",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "1",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Some",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "text",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "below",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "header.",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::Bold,
                expected_literal: "**",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Bold",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Text",
            },
            TestCase {
                expected_type: TokenType::Bold,
                expected_literal: "**",
            },
            TestCase {
                expected_type: TokenType::Italic,
                expected_literal: "_",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Italic",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Text",
            },
            TestCase {
                expected_type: TokenType::Italic,
                expected_literal: "_",
            },
            TestCase {
                expected_type: TokenType::BoldItalic,
                expected_literal: "***",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Bold",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "and",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Italic",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Text",
            },
            TestCase {
                expected_type: TokenType::BoldItalic,
                expected_literal: "***",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::Strikethrough,
                expected_literal: "~~",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Strikethrough",
            },
            TestCase {
                expected_type: TokenType::Strikethrough,
                expected_literal: "~~",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::Header,
                expected_literal: "##",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Subheader",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "1",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "-",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Bullet",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "1",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "-",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Bullet",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "2",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Another",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "paragraph",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "here.",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::Header,
                expected_literal: "######",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Small",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Header",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "#######",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Invalid",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Header",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "End",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "of",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "file.",
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

    #[test]
    fn test_format() {
        let input = String::from("***bold and italic*** **bold** _italic_ ~~strikethrough~~");

        let tests = vec![
            TestCase {
                expected_type: TokenType::BoldItalic,
                expected_literal: "***",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "bold",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "and",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "italic",
            },
            TestCase {
                expected_type: TokenType::BoldItalic,
                expected_literal: "***",
            },
            TestCase {
                expected_type: TokenType::Bold,
                expected_literal: "**",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "bold",
            },
            TestCase {
                expected_type: TokenType::Bold,
                expected_literal: "**",
            },
            TestCase {
                expected_type: TokenType::Italic,
                expected_literal: "_",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "italic",
            },
            TestCase {
                expected_type: TokenType::Italic,
                expected_literal: "_",
            },
            TestCase {
                expected_type: TokenType::Strikethrough,
                expected_literal: "~~",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "strikethrough",
            },
            TestCase {
                expected_type: TokenType::Strikethrough,
                expected_literal: "~~",
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

    #[test]
    fn test_newlines() {
        let input = String::from("\n\nLine 1\n\nLine 2\n\n");

        let tests = vec![
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Line",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "1",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Line",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "2",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
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

    #[test]
    fn test_header() {
        let input = String::from("#\n######\n#######");

        let tests = vec![
            TestCase {
                expected_type: TokenType::Header,
                expected_literal: "#",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::Header,
                expected_literal: "######",
            },
            TestCase {
                expected_type: TokenType::NewLine,
                expected_literal: "\n",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "#######",
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

    #[test]
    fn test_lexer_word() {
        let input = String::from("A\t1234 Numbers_and_underscores4");

        let tests = vec![
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "A",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "1234",
            },
            TestCase {
                expected_type: TokenType::String,
                expected_literal: "Numbers_and_underscores4",
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
