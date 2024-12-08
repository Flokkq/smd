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
        let mut html = String::with_capacity(tokens.len() * 100);
        let mut quote_level = 0;
        let mut in_paragraph = false;
        let mut token_iter = tokens.iter().peekable();

        // multi-liners
        while let Some(token) = token_iter.next() {
            match token {
                Token::Plaintext(t) if t.trim().is_empty() => {} // ignore
                Token::Plaintext(_)
                | Token::Italic(_)
                | Token::Bold(_)
                | Token::BoldItalic(_)
                | Token::Strikethrough(_)
                | Token::Link(_, _, _)
                    if !in_paragraph =>
                {
                    for _i in 0..quote_level {
                        html.push_str("</blockquote>");
                        quote_level -= 1;
                    }
                    in_paragraph = true;
                    html.push_str("<p>")
                }
                _ => {}
            }

            match token {
                Token::Plaintext(t) => {
                    let mut t: String = t.to_string();
                    if t.trim().is_empty() {
                        continue;
                    }

                    t.rfind('\n').map(|n| {
                        let (_before, after) = t.split_at(n);
                        if after.chars().all(|c| c.is_whitespace()) {
                            t = t.trim_end_matches(after).to_string();
                        }
                    });

                    if t.contains("[^") && t.contains("]") {
                        let plaintext_tokens = t.split("[^");
                        let mut s = String::new();
                        let mut count = 1;
                        for tok in plaintext_tokens {
                            if tok.trim_end().ends_with("]") {
                                let tok = tok.trim_end().trim_end_matches(']');
                                s.push_str(format!(
                                    "<sup id=\"fnref:{reference}\" role=\"doc-noteref\"><a href=\"#fn:{reference}\" class=\"footnote\" rel=\"footnote\">{ref_count}</a></sup>", 
                                    reference = Self::sanitize_display_text(tok),
                                    ref_count = count).as_str());
                                count += 1;
                            } else {
                                s.push_str(tok)
                            }
                        }
                        html.push_str(&s);
                    } else {
                        html.push_str(&Self::sanitize_display_text(t.trim_start_matches('\n')))
                    }
                }
                _ => {}
            }
        }

        if in_paragraph {
            html.push_str("</p>\n");
        }
        if quote_level > 0 {
            for _i in (0..quote_level).rev() {
                html.push_str("</blockquote>\n");
            }
        }
        if html.chars().last().unwrap_or(' ') != '\n' {
            html.push('\n');
        }

        html
    }

    pub(crate) fn sanitize_display_text(source: &str) -> String {
        source
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
            .replace('[', "&lbrack;")
            .replace(']', "&rbrack;")
            .replace('{', "&lbrace;")
            .replace('}', "&rbrace;")
            .replace('|', "&mid;")
            .replace('\\', "")
            .replace('~', "&tilde;")
            .replace(')', "&#41;")
            .replace('(', "&#40;")
    }
}
