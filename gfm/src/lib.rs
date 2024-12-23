mod iter;
mod lexer;
mod token;

use lexer::Lexer;
use log::debug;
use token::Token;

use crate::token::TaskBox;

pub struct Parser {}

impl Parser {
	/// Converts a Markdown string into sanitized HTML.
	///
	/// This function takes a Markdown-formatted string and returns its HTML
	/// representation, ensuring that the output is safe from script injection.
	///
	/// ### Example
	///
	/// ```rust
	/// let markdown = "| foo | bar |\n| --- | --- |\n| baz | bim |\n";
	/// let html = Parser::render(markdown);
	/// let expected_html = "\
	/// <table>\n<thead>\n<tr>\n<th>foo</th>\n<th>bar</th>\n</tr>\n</thead>\n<tbody>\\
	///                      n<tr>\n<td>baz</td>\n<td>bim</td>\n</tr>\n</tbody>\\
	///                      n</table>\n";
	/// assert_eq!(html, expected_html);
	/// ```
	pub fn render(source: &str) -> String {
		debug!("Rendering source of length: {}", source.len());
		return Self::parse(&Self::lex(source, &[]));
	}

	pub(crate) fn render_ignore(source: &str, ignore: &[char]) -> String {
		return Self::parse(&Self::lex(source, ignore));
	}

	pub(crate) fn lex<'a>(source: &'a str, ignore: &[char]) -> Vec<Token<'a>> {
		debug!("Lexing source with ignore list: {:?}", ignore);
		let mut l = Lexer::new(source);
		let mut tokens = Vec::new();

		while let Some(token) = l.next_token(ignore, &tokens) {
			tokens.push(token);
		}

		debug!("Lexing completed, total tokens: {}", tokens.len());
		tokens
	}

	fn parse<'a>(tokens: &[Token<'a>]) -> String {
		debug!("Parsing {} tokens", tokens.len());
		let mut html = String::with_capacity(tokens.len() * 100);
		let mut quote_level = 0;
		let mut in_task_list = false;
		let mut in_paragraph = false;
		let mut in_ordered_list = false;
		let mut in_unordered_list = false;
		let mut token_iter = tokens.iter().peekable();

		// multi-liners
		while let Some(token) = token_iter.next() {
			debug!("Processing token: {:?}", token);
			match token {
				Token::Plaintext(t) if t.trim().is_empty() => {} // ignore
				Token::Tab | Token::DoubleTab => {}
				Token::OrderedListEntry(_) |
				Token::UnorderedListEntry(_) |
				Token::Newline
					if in_ordered_list | in_unordered_list => {}
				Token::TaskListItem(_, _) | Token::Newline if in_task_list => {}
				Token::Plaintext(_) |
				Token::Italic(_) |
				Token::Bold(_) |
				Token::BoldItalic(_) |
				Token::Strikethrough(_) |
				Token::Link(_, _, _)
					if !in_paragraph =>
				{
					for _i in 0..quote_level {
						html.push_str("</blockquote>");
						quote_level -= 1;
					}
					in_paragraph = true;
					html.push_str("<p>")
				}
				_ if in_ordered_list => {
					in_ordered_list = false;
					html.push_str("</ol>\n");
					if !in_paragraph {
						in_paragraph = true;
						html.push_str("<p>")
					}
				}
				_ if in_unordered_list => {
					in_unordered_list = false;
					html.push_str("</ul>\n");
					if !in_paragraph {
						in_paragraph = true;
						html.push_str("<p>")
					}
				}
				_ if in_task_list => {
					in_task_list = false;
					html.push_str("</ul>\n");
					if !in_paragraph {
						in_paragraph = true;
						html.push_str("<p>")
					}
				}
				Token::CodeBlock(_, _) |
				Token::Newline |
				Token::Header(_, _, _)
					if in_paragraph =>
				{
					in_paragraph = false;
					html.push_str("</p>\n")
				}
				Token::BlockQuote(_, _) | Token::Newline if quote_level > 0 => {
				}
				Token::CodeBlock(_, _) |
				Token::Newline |
				Token::Header(_, _, _)
					if in_paragraph =>
				{
					in_paragraph = false;
					html.push_str("</p>\n")
				}
				Token::Plaintext(_) |
				Token::Italic(_) |
				Token::Bold(_) |
				Token::BoldItalic(_) |
				Token::Strikethrough(_) |
				Token::Link(_, _, _)
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
								s.push_str(
									format!(
										"<sup id=\"fnref:{reference}\" \
										 role=\"doc-noteref\"><a \
										 href=\"#fn:{reference}\" \
										 class=\"footnote\" \
										 rel=\"footnote\">{ref_count}</a></\
										 sup>",
										reference =
											Self::sanitize_display_text(tok),
										ref_count = count
									)
									.as_str(),
								);
								count += 1;
							} else {
								s.push_str(tok)
							}
						}
						html.push_str(&s);
					} else {
						html.push_str(&Self::sanitize_display_text(
							t.trim_start_matches('\n'),
						))
					}
				}
				Token::Header(l, t, lbl) => {
					match lbl {
						Some(lbl_text) => html.push_str(
							format!(
								"<h{level} id=\"{id}\">{text}</h{level}>\n",
								level = l,
								text = t,
								id = Self::sanitize_display_text(
									&lbl_text.replace(" ", "-")
								) /* TODO:
								   * is id necessary? */
							)
							.as_str(),
						),
						None => html.push_str(
							format!(
								"<h{level}>{text}</h{level}>\n",
								level = l,
								text = t
							)
							.as_str(),
						),
					};
				}
				Token::OrderedListEntry(t) => {
					if in_ordered_list == false {
						in_ordered_list = true;
						html.push_str(format!("<ol>\n").as_str())
					}
					html.push_str(
						format!(
							"<li>\n{}</li>\n",
							Self::sanitize_display_text(t)
						)
						.as_str(),
					)
				}
				Token::UnorderedListEntry(toks) => {
					if in_unordered_list == false {
						in_unordered_list = true;
						html.push_str("<ul>\n")
					}

					html.push_str(format!("<li>").as_str());
					if toks
						.into_iter()
						.all(|t| matches!(t, Token::Plaintext(_)))
					{
						html.push_str(format!("\n").as_str());
					}
					for token in toks.iter() {
						match token {
							Token::Plaintext(text)
								if text.starts_with("\t\t") =>
							{
								html.push_str(
									&Self::render(
										&text[1..].trim_start_matches(" "),
									)
									.replace("<pre><code>", "<pre><code>  "),
								);
							}
							Token::Plaintext(text) => {
								let text = &Self::render(
									&text.trim_start_matches(" "),
								)
								.replace("<pre><code>", "<pre><code>  ");
								html.push_str(text);
							}
							_ => {}
						}
					}
					html.push_str(format!("</li>\n").as_str());
				}
				Token::Italic(t) => html.push_str(
					format!("<em>{}</em>", Self::sanitize_display_text(t))
						.as_str(),
				),
				Token::Bold(t) => html.push_str(
					format!(
						"<strong>{}</strong>",
						Self::sanitize_display_text(t)
					)
					.as_str(),
				),
				Token::BoldItalic(t) => html.push_str(
					format!(
						"<strong><em>{}</em></strong>",
						Self::sanitize_display_text(t)
					)
					.as_str(),
				),
				Token::BlockQuote(l, t) => {
					if in_paragraph {
						html.push_str("</p>");
						in_paragraph = false;
					}
					match quote_level {
						_ if l == &quote_level => {}
						_ if l < &quote_level => {
							let diff = quote_level - l;
							quote_level = *l;
							for _i in 0..diff {
								html.push_str("</blockquote>");
							}
						}
						_ if l > &quote_level => {
							let diff = l - quote_level;
							quote_level = *l;
							for _i in 0..diff {
								html.push_str("<blockquote>\n");
							}
						}
						_ => {}
					}
					if !t.is_empty() {
						html.push_str(
							&Self::render(&Self::sanitize_display_text(
								&t.trim_start_matches(" "),
							))
							.replace("\t", "  "),
						);
					}
				}
				Token::TaskListItem(c, t) => {
					if in_task_list == false {
						in_task_list = true;
						html.push_str("<ul class=\"contains-task-list\">")
					}
					match c {
						TaskBox::Checked => html.push_str(
							format!(
								"<li class=\"task-list-item\"><input \
								 type=\"checkbox\" \
								 class=\"task-list-item-checkbox\" \
								 checked=\"\">{}</li>",
								Self::sanitize_display_text(t)
							)
							.as_str(),
						),
						TaskBox::Unchecked => html.push_str(
							format!(
								"<li class=\"task-list-item\"><input \
								 type=\"checkbox\" \
								 class=\"task-list-item-checkbox\">{}</li>",
								Self::sanitize_display_text(t)
							)
							.as_str(),
						),
					}
				}
				Token::HorizontalRule => html.push_str("<hr />\n"),
				Token::Newline => {}
				Token::Tab => html.push('\t'),
				Token::DoubleTab => html.push_str("\t\t"),
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
		if in_task_list | in_unordered_list {
			html.push_str("</ul>");
		}
		if in_ordered_list {
			html.push_str("</ol>");
		}
		if html.chars().last().unwrap_or(' ') != '\n' {
			html.push('\n');
		}

		debug!("Parsing completed");
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
