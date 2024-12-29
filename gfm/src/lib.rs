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
		Self::parse(&Self::lex(source, &[]))
	}

	pub(crate) fn render_ignore(source: &str, ignore: &[char]) -> String {
		Self::parse(&Self::lex(source, ignore))
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

	fn parse(tokens: &[Token<'_>]) -> String {
		debug!("Parsing {} tokens", tokens.len());
		let mut html = String::with_capacity(tokens.len() * 100);
		let mut quote_level = 0;
		let mut in_task_list = false;
		let mut in_paragraph = false;
		let mut in_ordered_list = false;
		let mut in_unordered_list = false;
		let mut in_code = false;
		let mut references = Vec::new();
		let mut token_iter = tokens.iter().peekable();

		// multi-liners
		while token_iter.peek().is_some() {
			let token = token_iter.next().unwrap();

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
				Token::Code(_) if !in_code => {
					html.push_str("<pre><code>");
					in_code = true;
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
					if !in_ordered_list {
						in_ordered_list = true;
						html.push_str("<ol>\n".to_string().as_str())
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
					if !in_unordered_list {
						in_unordered_list = true;
						html.push_str("<ul>\n")
					}

					html.push_str("<li>".to_string().as_str());
					if toks.iter().all(|t| matches!(t, Token::Plaintext(_))) {
						html.push_str("\n".to_string().as_str());
					}
					for token in toks.iter() {
						match token {
							Token::Plaintext(text)
								if text.starts_with("\t\t") =>
							{
								html.push_str(
									&Self::render(
										text[1..].trim_start_matches(" "),
									)
									.replace("<pre><code>", "<pre><code>  "),
								);
							}
							Token::Plaintext(text) => {
								let text =
									&Self::render(text.trim_start_matches(" "))
										.replace(
											"<pre><code>",
											"<pre><code>  ",
										);
								html.push_str(text);
							}
							_ => {}
						}
					}
					html.push_str("</li>\n".to_string().as_str());
				}
				Token::Image(l, t) => match (l, t) {
					(l, None) if l.trim() == "" => {
						html.push_str("<p><img src=\"data:,\"></p>")
					}
					(l, Some(t)) if l.trim() == "" => html.push_str(
						format!(
							"<p><img src=\"data:,\" alt=\"{text}\"></p>",
							text = Self::sanitize_display_text(t)
						)
						.as_str(),
					),
					(l, None) => html.push_str(
						format!(
							"<p><img src=\"{link}\"> \
							 referrerpolicy=\"no-referrer\"></p>",
							link = l
						)
						.as_str(),
					),
					(l, Some(t)) => html.push_str(
						format!(
							"<p><img src=\"{link}\" alt=\"{text}\" \
							 referrerpolicy=\"no-referrer\"></p>",
							link = l,
							text = Self::sanitize_display_text(t)
						)
						.as_str(),
					),
				},
				Token::Link(l, t, ht) => match (t, ht) {
					(Some(t), Some(ht)) => html.push_str(
						format!(
							"<a href=>\"{link}\" title=\"{hover}\">{text}</a>",
							link = l,
							text = Self::sanitize_display_text(t),
							hover = ht
						)
						.as_str(),
					),
					(Some(t), None) => html.push_str(
						format!(
							"<a href=\"{link}\">{text}</a>",
							link = l,
							text = Self::sanitize_display_text(t)
						)
						.as_str(),
					),
					(None, Some(ht)) => html.push_str(
						format!(
							"<a href=\"{link}\" title=\"{hover}\">{link}</a>",
							link = l,
							hover = Self::sanitize_display_text(ht)
						)
						.as_str(),
					),
					(None, None) => html.push_str(
						format!(
							"<a href=\"{link}\">{display}</a>",
							link = l,
							display = l.fmt_unsafe()
						)
						.as_str(),
					),
				},
				Token::Detail(summary, inner_tokens) => {
					if in_paragraph {
						html.push_str("</p>\n");
						in_paragraph = false;
					}
					let inner_html = Self::parse(inner_tokens);
					html.push_str(
						format!(
							"<details>\n<summary>{sum}</summary>\n{in_html}\\
							 n</details>",
							sum = Self::sanitize_display_text(summary),
							in_html = inner_html
						)
						.as_str(),
					);
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
								t.trim_start_matches(" "),
							))
							.replace("\t", "  "),
						);
					}
				}
				Token::TaskListItem(c, t) => {
					if !in_task_list {
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
				Token::Table(headings, rows) => {
					if headings.len() != rows[0].len() {
						continue;
					}
					html.push_str(
						"<table class=\"table \
						 table-bordered\">\n\t<thead>\n\t<tr>\n",
					);
					for h in headings.into_iter() {
						html.push_str(
							format!(
								"\t\t<th style=\"text-align: \
								 {align}\">{heading}</th>",
								heading = Self::sanitize_display_text(&h.1),
								align = h.0
							)
							.as_str(),
						);
					}
					html.push_str("\t</tr>\n\t</thead>\n\t<tbody>");
					for row in rows.iter() {
						html.push_str("\n\t<tr>");
						for elem in row.iter() {
							let mut row_string = String::new();
							for token in elem.1.iter() {
								match token {
									Token::Plaintext(s) => row_string.push_str(
										&Self::sanitize_display_text(&s),
									),
									Token::Italic(t) => row_string.push_str(
										format!(
											"<em>{}</em>",
											Self::sanitize_display_text(t)
										)
										.as_str(),
									),
									Token::Bold(t) => row_string.push_str(
										format!(
											"<strong>{}</strong>",
											Self::sanitize_display_text(t)
										)
										.as_str(),
									),
									Token::BoldItalic(t) => row_string
										.push_str(
											format!(
												"<strong><em>{}</em></strong>",
												Self::sanitize_display_text(t)
											)
											.as_str(),
										),
									Token::LineBreak => {
										row_string.push_str("<br>")
									}
									Token::HorizontalRule => {
										row_string.push_str("<hr />")
									}
									Token::Strikethrough(t) => row_string
										.push_str(
											format!(
												"<strike>{}</strike>",
												Self::sanitize_display_text(t)
											)
											.as_str(),
										),
									_ => row_string
										.push_str(&Self::parse(&elem.1)),
								}
							}
							html.push_str(
								format!(
									"\n\t\t<td style=\"text-align: \
									 {align}\">{row_text}</td>",
									align = elem.0,
									row_text = row_string
								)
								.as_str(),
							);
						}
						html.push_str("\n\t</tr>");
					}
					html.push_str("\n\t</tbody>\n</table>");
				}
				Token::Strikethrough(t) => html.push_str(
					format!(
						"<strike>{}</strike>",
						Self::sanitize_display_text(t)
					)
					.as_str(),
				),
				Token::Code(t) => html.push_str(
					format!("{}", Self::sanitize_display_text(t)).as_str(),
				),
				Token::HorizontalRule => html.push_str("<hr />\n"),
				Token::Newline => {}
				Token::Tab => html.push('\t'),
				Token::DoubleTab => html.push_str("\t\t"),
				Token::Footnote(ref_id, text) => {
					references.push((ref_id, text));
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
		if in_task_list | in_unordered_list {
			html.push_str("</ul>");
		}
		if in_ordered_list {
			html.push_str("</ol>");
		}
		if html.chars().last().unwrap_or(' ') != '\n' {
			html.push('\n');
		}
		if in_code && !matches!(token_iter.peek(), Some(Token::Code(_))) {
			match html.chars().last().unwrap() {
				'\n' => {}
				_ => html.push('\n'),
			}
			html.push_str("</code></pre>");
		}

		if references.len() > 0 {
			html.push_str("<div class=\"footnotes\" role=\"doc-endnotes\">\n");
			html.push_str("\t<ol>\n");
			for reference in references.iter() {
				html.push_str("\t\t<li id=\"fn:1\" role=\"doc-endnote\">");
				html.push_str(
					format!(
						"\t\t\t<p>{ref_text}<a href=\"#fnref:{ref_count}\" \
						 class=\"reversefootnote\" \
						 role=\"doc-backlink\">↩</a></p>",
						ref_count = Self::sanitize_display_text(reference.0),
						ref_text = Self::sanitize_display_text(reference.1)
					)
					.as_str(),
				);
				html.push_str("\t\t</li>");
			}
			html.push_str("\t</ol>\n");
			html.push_str("</div>\n");
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
