#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Header(usize),
    Paragraph,
    Text,
    EOF,
    ILLEGAL,
}
