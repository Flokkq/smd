use core::fmt;

use log::{
	debug,
	warn,
};

use super::{
	iter::MarkdownIter,
	parser::Parser,
	token::{
		TaskBox,
		Token,
	},
};

pub struct Lexer<'a> {
	iter: MarkdownIter<'a>,
}

#[derive(Debug)]
pub(crate) struct ParseError<'a> {
	pub(crate) content: &'a str,
}

impl<'a> fmt::Display for ParseError<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.content)
	}
}

impl<'a> Lexer<'a> {
	/// Initializes a new Lexer with the given input.
	pub fn new(input: &'a str) -> Self {
		debug!("Initializing Lexer with input of length: {}", input.len());
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
			debug!("Processing character: {:?}", self.iter.peek());
			match self.iter.peek().unwrap() {
				"#" if !ignore.contains(&'#') => {
					return match self.lex_heading() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing heading: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					};
				}
				"\n" => {
					return match self.lex_newlines() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing newline: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				" " | "\t" => {
					return match self.lex_tabs_spaces(&tokens) {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing whitespaces: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				"1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" => {
					return match self.lex_numbers() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing number: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				"*" | "_" => {
					return match self.lex_asterisk_underscore() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!(
								"Error while lexing asterisks and \
								 underscores: {}",
								e
							);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				">" => {
					return match self.lex_blockquotes() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing block quotes: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				// Parse "\" to escape a markdown control character
				"\\" => {
					return match self.lex_escaped_character() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!(
								"Error while lexing escaped character: {}",
								e
							);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				_ => {
					if let Some(c) = self.iter.next() {
						return Some(Token::Plaintext(c.to_string()));
					}
				}
			}
		}

		debug!("End of input reached, no more tokens");
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

		if self.iter.next_if_eq(&" ").is_none() &&
			self.iter.next_if_eq(&"\t").is_none() &&
			self.iter.peek() != Some(&"\n")
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
				if s.len() >= 2 &&
					!s.contains("\t") &&
					self.iter.peek() == Some("\n") =>
			{
				return Ok(Token::LineBreak)
			}
			Some(_s) => {}
		}

		let whitespace = whitespace.unwrap_or("");
		let line = self.iter.consume_until_tail_is("\n").unwrap_or("");

		match whitespace {
			"    "
				if (matches!(tokens.last(), Some(Token::Plaintext(_))) &&
					line.contains('#')) =>
			{
				return Err(ParseError { content: line })
			}
			"    "
				if (matches!(tokens.last(), Some(Token::Newline)) &&
					line.contains('#')) =>
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

	fn lex_numbers(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		let c = self.iter.next().unwrap();
		match self.iter.next_if_eq(".") {
			Some(".") => {
				if self.iter.next_if_eq(" ") != Some(&" ") {
					return Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					});
				}

				let s = self
					.iter
					.consume_while_case_holds(&|c| c != "\n")
					.unwrap_or("");
				return Ok(Token::OrderedListEntry(s.to_string()));
			}
			_ => return Err(ParseError { content: c }),
		}
	}

	fn lex_escaped_character(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		self.iter.next();
		if self.iter.peek() == Some(&"#") {
			let hashes = self
				.iter
				.consume_while_case_holds(&|c| c == "#")
				.unwrap_or("");
			return Ok(Token::Plaintext(hashes.to_string()));
		} else if self.iter.peek().is_some() {
			return Ok(Token::Plaintext(
				self.iter.next().unwrap_or_default().to_string(),
			));
		}

		return Err(ParseError { content: "EOF" });
	}

	fn lex_asterisk_underscore(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		let asterunds = self
			.iter
			.consume_while_case_holds(&|c| c == "*" || c == "_" || c == "\t")
			.unwrap_or("");
		if asterunds.len() == 1 && self.iter.next_if_eq(&" ").is_some() {
			let s = self
				.iter
				.consume_while_case_holds(&|c| c != "\n")
				.unwrap_or("");
			self.iter.next();
			return Ok(Token::UnorderedListEntry(vec![Token::Plaintext(
				s.to_string(),
			)]));
		}
		if asterunds.chars().all(|x| x == '*') &&
			self.iter.peek() == Some(&"\n")
		{
			return Ok(Token::HorizontalRule);
		}
		match asterunds.len() {
			1 => {
				let s = self
					.iter
					.consume_while_case_holds(&|c| c != "*" && c != "_")
					.unwrap_or("");
				if self.iter.peek() != Some("*") ||
					self.iter.peek() != Some(&"_")
				{
					self.iter.next();
					return Ok(Token::Italic(s.to_string()));
				} else {
					return Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					});
				}
			}
			2 => {
				let s = self
					.iter
					.consume_while_case_holds(&|c| c != "*" && c != "_")
					.unwrap_or("");
				let trailing_astunds = self
					.iter
					.consume_while_case_holds(&|c| c == "*" || c == "_")
					.unwrap_or("");
				if trailing_astunds.len() == 2 {
					return Ok(Token::Bold(s.to_string()));
				} else {
					return Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					});
				}
			}
			3 => {
				let s = self
					.iter
					.consume_while_case_holds(&|c| c != "*" && c != "_")
					.unwrap_or("");
				let trailing_astunds = self
					.iter
					.consume_while_case_holds(&|c| c == "*" || c == "_")
					.unwrap_or("");
				if trailing_astunds.len() == 3 {
					return Ok(Token::BoldItalic(s.to_string()));
				} else {
					return Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					});
				}
			}
			_ => {
				if asterunds.replace("\t", "").chars().all(|x| x == '*') ||
					asterunds.chars().all(|x| x == '_')
				{
					return Ok(Token::HorizontalRule);
				} else {
					return Err(ParseError { content: asterunds });
				}
			}
		}
	}

	pub fn lex_blockquotes(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let right_arrows = self
			.iter
			.consume_while_case_holds(&|c| c == ">")
			.unwrap_or("");
		match self.iter.peek() {
			Some(" ") | Some("\t") => {}
			_ => {
				return Err(ParseError {
					content: right_arrows,
				})
			}
		}
		let s = self
			.iter
			.consume_while_case_holds(&|c| c != "\n")
			.unwrap_or("");
		self.iter.next_if_eq(&"\n");
		Ok(Token::BlockQuote(right_arrows.len() as u8, s.to_string()))
	}
}
