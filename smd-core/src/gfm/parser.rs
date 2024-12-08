use super::{lexer::Lexer, token::Token};

pub struct Parser {}

impl Parser {
    /// Render HTML from a source markdown string
    /// Output is sanitized to prevent script injection
    pub fn render(source: &str) -> String {
        return Self::parse(&Self::lex(source, &[]));
    }

    pub(crate) fn render_ignore(source: &str, ignore: &[char]) -> String {
        return Self::parse(&Self::lex(source, ignore));
    }
    
    fn lex<'a>(source: &'a str, ignore: &[char]) -> Vec<Token<'a>> {
        let mut l = Lexer::new(source);
        let mut tokens = Vec::new();

        while let Some(token) = l.next_token(ignore) {
            tokens.push(token);
        }

        tokens
    }

    fn parse<'a>(tokens: &[Token<'a>]) -> String {
    }
}
