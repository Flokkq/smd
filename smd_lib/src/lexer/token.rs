#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Header,
    Paragraph,
    Word, // [A-Za-z0-9_]
    EOF,
    ILLEGAL,
}
