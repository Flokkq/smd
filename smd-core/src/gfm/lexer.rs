use super::{iter::MarkdownIter, token::Token};

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
    pub fn next_token(&mut self, ignore: &[char]) -> Option<Token> {
        while self.iter.peek().is_some() {
            match self.iter.peek().unwrap() {
                "#" if !ignore.contains(&'#') => {
                    return match self.lex_heading() {
                        Ok(t) => Some(t),
                        Err(e) => Some(Token::Plaintext(e.content.to_string())),
                    };
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

    fn lex_heading(&mut self) -> Result<Token, ParseError<'a>> {
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

        let line_without_optional_trailing_hash_sequence = match line.trim_end().rsplit_once(' ') {
            Some((left, right)) => match right.chars().all(|c| c == '#') {
                true => left,
                false => line,
            },
            None => line,
        };

        if line.chars().all(|c| c == '#') {
            return Ok(Token::Header(hashes.len(), "".to_string(), None));
        }

        // TODO: render inline styles
        return Ok(Token::Header(
            hashes.len(),
            line_without_optional_trailing_hash_sequence.to_string(),
            None,
        ));
    }
}
