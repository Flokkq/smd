use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    /// String: Body of unstructured text
    Plaintext(String),
    /// u8: Header level (1..=6). str: Header text. Option<str>: html label
    Header(usize, String, Option<String>),
    /// str: Text to be italicized
    Italic(String),
    /// str: Text to be bolded
    Bold(String),
    /// str: Text to be bolded and italicized
    BoldItalic(String),
    /// Corresponds to a newline character
    Newline,
    /// Used for control flow. Not directly rendered
    Tab,
}

impl<'a> fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Plaintext(t) => {
                write!(f, "{:?}", t)
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}
