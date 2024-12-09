use super::{iter::MarkdownIter, parser::Parser, token::Token};

pub struct Lexer<'a> {
    iter: MarkdownIter<'a>,
}

#[derive(Debug)]
pub(crate) struct ParseError<'a> {
    pub(crate) content: &'a str,
}

impl<'a> Lexer<'a> {
    /// Initializes a new Lexer with the given input.
    pub fn new(input: &'a str) -> Self {
        Lexer {
            iter: MarkdownIter::new(input),
        }
    }

    /// Produces the next token or returns None when done.
    pub fn next_token(
        &mut self,
        ignore: &[char],
        tokens: &Vec<Token>,
    ) -> Option<Token<'a>> {
        while self.iter.peek().is_some() {
            match self.iter.peek().unwrap() {
                "#" if !ignore.contains(&'#') => {
                    return match self.lex_heading() {
                        Ok(t) => Some(t),
                        Err(e) => Some(Token::Plaintext(e.content.to_string())),
                    };
                }
                "\n" => {
                    return match self.lex_newlines() {
                        Ok(t) => Some(t),
                        Err(e) => Some(Token::Plaintext(e.content.to_string())),
                    }
                }
                " " | "\t" => {
                    return match self.lex_tabs_spaces(&tokens) {
                        Ok(t) => Some(t),
                        Err(e) => Some(Token::Plaintext(e.content.to_string())),
                    }
                }
                _ => {
                    if let Some(c) = self.iter.next() {
                        return Some(Token::Plaintext(c.to_string()));
                    }
                }
            }
        }
        None
    }

    fn lex_heading(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        let hashes = self
            .iter
            .consume_while_case_holds(&|c| c == "#")
            .unwrap_or("");

        if hashes.len() > 6 {
            return Err(ParseError { content: hashes });
        }

        if self.iter.next_if_eq(&" ").is_none()
            && self.iter.next_if_eq(&"\t").is_none()
            && self.iter.peek() != Some(&"\n")
        {
            return Err(ParseError { content: hashes });
        }

        let line = self
            .iter
            .consume_while_case_holds(&|c| c != "\n")
            .unwrap_or("");

        let line_without_optional_trailing_hash_sequence =
            match line.trim_end().rsplit_once(' ') {
                Some((left, right)) => match right.chars().all(|c| c == '#') {
                    true => left,
                    false => line,
                },
                None => line,
            };

        if line.chars().all(|c| c == '#') {
            return Ok(Token::Header(hashes.len(), "".to_string(), None));
        }

        let parsed_line = Parser::render_ignore(
            line_without_optional_trailing_hash_sequence
                .trim_end_matches(&[' ', '\t']),
            &['#'],
        )
        .to_string();

        return Ok(Token::Header(hashes.len(), parsed_line, None));
    }

    fn lex_newlines(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        match self.iter.consume_while_case_holds(&|c| c == "\n") {
            Some(s) if s.len() >= 2 => return Ok(Token::Newline),
            Some(s) if s.len() < 2 => return Err(ParseError { content: s }),
            _ => return Err(ParseError { content: "" }),
        }
    }

    fn lex_tabs_spaces(
        &mut self,
        tokens: &Vec<Token>,
    ) -> Result<Token<'a>, ParseError<'a>> {
        let start_index = self.iter.get_index();
        let whitespace = self
            .iter
            .consume_while_case_holds(&|c| c == "\t" || c == " ");

        match whitespace {
            None => return Err(ParseError { content: "" }),
            Some(s) if (1..=3).contains(&s.len()) && !s.contains("\t") => {
                return Err(ParseError { content: s })
            }
            Some(s)
                if s.len() >= 2
                    && !s.contains("\t")
                    && self.iter.peek() == Some("\n") =>
            {
                return Ok(Token::LineBreak)
            }
            Some(_s) => {}
        }

        let whitespace = whitespace.unwrap_or("");
        let line = self.iter.consume_until_tail_is("\n").unwrap_or("");

        match whitespace {
            "    "
                if (matches!(tokens.last(), Some(Token::Plaintext(_)))
                    && line.contains('#')) =>
            {
                return Err(ParseError { content: line })
            }
            "    "
                if (matches!(tokens.last(), Some(Token::Newline))
                    && line.contains('#')) =>
            {
                return Err(ParseError { content: line })
            }
            "\t" if matches!(tokens.last(), Some(Token::Code(_))) => {
                return Ok(Token::Code(line.to_string()))
            }
            "\t" | "    " | "  \t" => return Ok(Token::Code(line.to_string())),
            "\t\t" => {
                return Ok(Token::Code("\t".to_owned() + &line.to_string()))
            }
            _ => {}
        }

        if self.iter.peek() == Some("\t") || self.iter.peek() == Some(" ") {
            match self.lex_tabs_spaces(tokens) {
                Ok(Token::CodeBlock(_content, _lang)) => {
                    return Ok(Token::CodeBlock(
                        self.iter
                            .get_substring_from(start_index)
                            .unwrap_or("")
                            .to_string(),
                        "".to_string(),
                    ))
                }
                Err(e) => return Err(e),
                Ok(_) => return Err(ParseError { content: "" }),
            }
        }
        return Err(ParseError {
            content: self.iter.get_substring_from(start_index).unwrap_or(""),
        });
    }
}
