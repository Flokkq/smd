use core::fmt;

use log::{
	debug,
	warn,
};

use crate::{
	token::validate_link,
	Parser,
};

use super::{
	iter::MarkdownIter,
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

impl fmt::Display for ParseError<'_> {
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
		while let Some(ch) = self.iter.peek() {
			debug!("Processing character: {:?}", self.iter.peek());
			match ch {
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
					return match self.lex_tabs_spaces(tokens) {
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
				"-" | "+" => {
					return match self.lex_plus_minus() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing plus or minus: {}", e);
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
				"~" => {
					return match self.lex_tilde() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while tilde: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				"`" => {
					return match self.lex_backticks() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing backticks: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				"[" => {
					return match self.lex_links() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing link: {}", e);
							Some(Token::Plaintext(e.content.to_string()))
						}
					}
				}
				"!" => {
					return match self.lex_images() {
						Ok(t) => Some(t),
						Err(e) => {
							warn!("Error while lexing image: {}", e);
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
		if self.iter.next_if_eq(" ").is_none() &&
			self.iter.next_if_eq("\t").is_none() &&
			self.iter.peek() != Some("\n")
		{
			return Err(ParseError { content: hashes });
		}
		let mut line = self
			.iter
			.consume_while_case_holds(&|c| c != "\n")
			.unwrap_or("");
		if line.contains("{#") && line.contains('}') {
			let (heading, _title) = line.split_once("{").unwrap_or(("", ""));
			line = line
				.strip_prefix(heading)
				.unwrap()
				.strip_prefix("{#")
				.unwrap()
				.strip_suffix("}")
				.unwrap();
		}
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
				.trim_end_matches([' ', '\t']),
			&['#'],
		)
		.strip_prefix("<p>")
		.unwrap_or("")
		.strip_suffix("</p>\n")
		.unwrap_or("")
		.trim()
		.to_string();

		Ok(Token::Header(hashes.len(), parsed_line, None))
	}

	fn lex_newlines(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		match self.iter.consume_while_case_holds(&|c| c == "\n") {
			Some(s) if s.len() >= 2 => Ok(Token::Newline),
			Some(s) if s.len() < 2 => Err(ParseError { content: s }),
			_ => Err(ParseError { content: "" }),
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
			"\t\t" => return Ok(Token::Code("\t".to_owned() + line)),
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
		Err(ParseError {
			content: self.iter.get_substring_from(start_index).unwrap_or(""),
		})
	}

	fn lex_numbers(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		let c = self.iter.next().unwrap();
		match self.iter.next_if_eq(".") {
			Some(".") => {
				if self.iter.next_if_eq(" ") != Some(" ") {
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
				Ok(Token::OrderedListEntry(s.to_string()))
			}
			_ => Err(ParseError { content: c }),
		}
	}

	fn lex_escaped_character(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		self.iter.next();
		if self.iter.peek() == Some("#") {
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

		Err(ParseError { content: "EOF" })
	}

	fn lex_asterisk_underscore(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		let asterunds = self
			.iter
			.consume_while_case_holds(&|c| c == "*" || c == "_" || c == "\t")
			.unwrap_or("");
		if asterunds.len() == 1 && self.iter.next_if_eq(" ").is_some() {
			let s = self
				.iter
				.consume_while_case_holds(&|c| c != "\n")
				.unwrap_or("");
			self.iter.next();
			return Ok(Token::UnorderedListEntry(vec![Token::Plaintext(
				s.to_string(),
			)]));
		}
		if asterunds.chars().all(|x| x == '*') && self.iter.peek() == Some("\n")
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
					self.iter.peek() != Some("_")
				{
					self.iter.next();
					Ok(Token::Italic(s.to_string()))
				} else {
					Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					})
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
					Ok(Token::Bold(s.to_string()))
				} else {
					Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					})
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
					Ok(Token::BoldItalic(s.to_string()))
				} else {
					Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					})
				}
			}
			_ => {
				if asterunds.replace("\t", "").chars().all(|x| x == '*') ||
					asterunds.chars().all(|x| x == '_')
				{
					Ok(Token::HorizontalRule)
				} else {
					Err(ParseError { content: asterunds })
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
		self.iter.next_if_eq("\n");
		Ok(Token::BlockQuote(right_arrows.len() as u8, s.to_string()))
	}

	pub(crate) fn lex_plus_minus(
		&mut self,
	) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		let s = self
			.iter
			.consume_while_case_holds(&|c| c == "-" || c == "+")
			.unwrap_or("");
		match s.len() {
			3..=usize::MAX => return Ok(Token::HorizontalRule),
			2 => {
				return Err(ParseError {
					content: self
						.iter
						.get_substring_from(start_index)
						.unwrap_or(""),
				})
			}
			1 => {}
			_ => {
				return Err(ParseError {
					content: "string length error",
				})
			}
		}
		let line_index = self.iter.get_index();
		let mut list_element_tokens = vec![Token::Plaintext(
			self.iter
				.consume_while_case_holds(&|c| c != "\n")
				.unwrap_or("")
				.to_string(),
		)];
		while self.iter.get_substring_ahead(3) == Some("\n\n\t") {
			self.iter.next();
			self.iter.next();
			self.iter.next();
			list_element_tokens.push(Token::Plaintext(
				self.iter
					.consume_while_case_holds(&|c| c != "\n")
					.unwrap_or("")
					.to_string(),
			));
		}
		let line = self.iter.get_substring_from(line_index).unwrap_or("");
		self.iter.next_if_eq("\n");
		if line.starts_with(" [ ] ") {
			Ok(Token::TaskListItem(
				TaskBox::Unchecked,
				line[5..].to_string(),
			))
		} else if line.starts_with(" [x] ") || line.starts_with(" [X] ") {
			return Ok(Token::TaskListItem(
				TaskBox::Checked,
				line[5..].to_string(),
			));
		} else {
			// List entries may contain other lists
			match self.iter.peek_line_ahead() {
				Some(s) if s.starts_with("  ") => {
					let line = self.iter.consume_line_ahead().unwrap_or("");
					list_element_tokens.append(&mut Parser::lex(line, &[]))
				}
				Some(s) if s.starts_with("\t") => {
					let line = self.iter.consume_line_ahead().unwrap_or("");
					list_element_tokens
						.append(&mut Parser::lex(&line[1..], &[]))
				}
				_ => {}
			}
			return Ok(Token::UnorderedListEntry(list_element_tokens));
		}
	}

	fn lex_tilde(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		let lead_tildes =
			match self.iter.consume_while_case_holds(&|s| s == "~") {
				Some(s) => s,
				None => {
					return Err(ParseError {
						content: "Failure to parse ~",
					})
				}
			};
		match lead_tildes.len() {
			1 => {
				return Err(ParseError {
					content: lead_tildes,
				})
			}
			2 => {
				let line = self
					.iter
					.consume_while_case_holds(&|s| s != "~")
					.unwrap_or("");
				let tail_tildes = self
					.iter
					.consume_while_case_holds(&|s| s == "~")
					.unwrap_or("");
				if lead_tildes.len() != tail_tildes.len() {
					return Err(ParseError {
						content: self
							.iter
							.get_substring_from(start_index)
							.unwrap_or(""),
					});
				}
				return Ok(Token::Strikethrough(line.to_string()));
			}
			_ => {
				return Err(ParseError {
					content: lead_tildes,
				})
			}
		}
	}

	fn lex_backticks(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		let leading_ticks = self
			.iter
			.consume_while_case_holds(&|c| c == "`")
			.unwrap_or("");
		let mut lang = "";
		if leading_ticks.len() == 3 {
			if self.iter.next_if_eq("\n") != Some(&"\n") {
				lang = self
					.iter
					.consume_while_case_holds(&|c| c != "\n")
					.unwrap_or("");
				self.iter.next();
			}
			let s = self
				.iter
				.consume_while_case_holds(&|c| c != "`")
				.unwrap_or("");
			let trailing_ticks = self
				.iter
				.consume_while_case_holds(&|c| c == "`")
				.unwrap_or("");
			if leading_ticks.len() != trailing_ticks.len() {
				return Err(ParseError {
					content: self
						.iter
						.get_substring_from(start_index)
						.unwrap_or(""),
				});
			} else {
				return Ok(Token::CodeBlock(s.to_string(), lang.to_string()));
			}
		}

		// let s = self.iter.consume_while_case_holds(&|c| c != "`" && c!=
		// "\n").unwrap_or("");
		let tail = &(0..leading_ticks.len() as u64)
			.map(|_| "`")
			.collect::<String>();
		let s = self.iter.consume_until_tail_is(tail).unwrap_or("");
		if !s.ends_with(tail) {
			return Err(ParseError {
				content: self
					.iter
					.get_substring_from(start_index)
					.unwrap_or(""),
			});
		} else {
			let s = s.trim_end_matches(tail);
			if s.starts_with(' ') && s.ends_with(' ') {
				return Ok(Token::Code(
					s.trim_start_matches(' ').trim_end_matches(' ').to_string(),
				));
			}
			return Ok(Token::Code(s.to_string()));
		}

		// leading_ticks.len() == 3. Check for lang
	}

	pub(crate) fn lex_links(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		if self.iter.next_if_eq("[") != Some(&"[") {
			return Err(ParseError { content: "" });
		}
		let title = self
			.iter
			.consume_while_case_holds(&|c| c != "]")
			.unwrap_or("");
		if self.iter.next_if_eq("]") != Some(&"]") {
			return Err(ParseError {
				content: self
					.iter
					.get_substring_from(start_index)
					.unwrap_or(""),
			});
		}
		// Parse footnotes big and small
		if title.starts_with("^") && self.iter.next_if_eq(":") == Some(&":") {
			let ref_id = title.strip_prefix("^").unwrap_or("");
			let note_index = self.iter.get_index();
			loop {
				self.iter.consume_while_case_holds(&|c| c != "\n");
				self.iter.next();
				if self.iter.peek() != Some(&" ") &&
					self.iter.peek() != Some(&"\t")
				{
					break;
				}
				if self.iter.next_if_eq("\t") == Some(&"\t") {
					continue;
				}
				if self.iter.peek() == Some(&" ") {
					let spaces = self
						.iter
						.consume_while_case_holds(&|c| c == " ")
						.unwrap_or("");
					match spaces.len() {
						2 | 4 => {}
						_ => {
							return Err(ParseError {
								content: self
									.iter
									.get_substring_from(start_index)
									.unwrap_or(""),
							})
						}
					}
					continue;
				}
				break;
			}
			if ref_id.contains(char::is_whitespace) {
				return Err(ParseError {
					content: self
						.iter
						.get_substring_from(start_index)
						.unwrap_or(""),
				});
			}
			return Ok(Token::Footnote(
				ref_id.to_string(),
				self.iter
					.get_substring_from(note_index)
					.unwrap_or("")
					.trim()
					.to_string(),
			));
		}
		if self.iter.next_if_eq("(") != Some(&"(") {
			return Err(ParseError {
				content: self
					.iter
					.get_substring_from(start_index)
					.unwrap_or(""),
			});
		}
		let link = self
			.iter
			.consume_while_case_holds(&|c| c != ")" && c != " ")
			.unwrap_or("");
		if self.iter.peek() != Some(&")") && self.iter.peek() != Some(&" ") {
			return Err(ParseError {
				content: self
					.iter
					.get_substring_from(start_index)
					.unwrap_or(""),
			});
		}
		if self.iter.next_if_eq(")") == Some(&")") {
			match validate_link(link) {
				Ok(vl) => {
					return Ok(Token::Link(vl, Some(title.to_string()), None))
				}
				Err(se) => {
					return Err(ParseError {
						content: &se.content,
					})
				}
			}
		}
		if self.iter.peek() == Some(&" ") {
			let hover = self
				.iter
				.consume_while_case_holds(&|c| c != ")")
				.unwrap_or("");

			self.iter.skip_while_true(|c| c != "\n");
			self.iter.next();
			match validate_link(link) {
				Ok(vl) => {
					return Ok(Token::Link(
						vl,
						Some(title.to_string()),
						Some(hover.to_string()),
					))
				}
				Err(se) => {
					return Err(ParseError {
						content: &se.content,
					})
				}
			}
		}
		Err(ParseError { content: "" })
	}

	pub(crate) fn lex_images(&mut self) -> Result<Token<'a>, ParseError<'a>> {
		let start_index = self.iter.get_index();
		if self.iter.next_if_eq("!") != Some(&"!") {
			return Err(ParseError { content: "" });
		}
		let link_result = self.lex_links();
		match link_result {
			Err(_e) => {
				return Err(ParseError {
					content: self
						.iter
						.get_substring_from(start_index)
						.unwrap_or(""),
				})
			}
			Ok(Token::Link(link, title, _)) => {
				return Ok(Token::Image(link.content.to_string(), title))
			}
			_ => {
				return Err(ParseError {
					content: "Non link token returned from lex_links",
				})
			}
		}
	}
}
