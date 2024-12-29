use core::fmt;

static COMMONMARK_SCHEME_ASCII: [char; 65] = [
	//https://spec.commonmark.org/0.30/#scheme
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
	'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
	'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
	't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8',
	'9', '0', '+', '.', '-',
];

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
	/// String: Body of unstructured text
	Plaintext(String),
	/// u8: Header level (1..=6). str: Header text. Option<str>: html label
	Header(usize, String, Option<String>),
	/// str: Text for list entry
	UnorderedListEntry(Vec<Token<'a>>),
	/// str: Text for list entry
	OrderedListEntry(String),
	/// str: Text to be italicized
	Italic(String),
	/// str: Text to be bolded
	Bold(String),
	/// str: Text to be bolded and italicized
	BoldItalic(String),
	/// Corresponds to a </br> html tag
	LineBreak,
	/// Corresponds to a newline character
	Newline,
	/// Corresponds to a <hr /> html tag
	HorizontalRule,
	/// Used for control flow. Not directly rendered
	Tab,
	/// Used for control flow. Not directly rendered
	DoubleTab,
	/// str: Text to be struck through
	Strikethrough(String),
	/// str: Text to be placed within an inline code tag. eg. <code>str</code>
	Code(String),
	/// First str: Text to be placed within a multi-line code tag. Second str:
	/// Language
	CodeBlock(String, String),
	/// u8: Block quote level. str: Block quote text
	BlockQuote(u8, String),
	/// str: Link. Option<str>: Title for link.
	Image(String, Option<String>),
	/// str: Link. First Option<str>: Title for link. Second Option<str>: Hover
	/// text
	Link(ValidURL<'a>, Option<String>, Option<String>),
	/// str: Summary. Vec<Token>: Tokens to be rendered in the collapsable
	/// section
	Detail(String, Vec<Token<'a>>),
	/// Tuple of Vec<(Alignment, str)>: Which defines the table header and
	/// Vec<Vec<(Alignment, Vec<Token>)>> which defines the rows
	Table(
		Vec<(Alignment, String)>,
		Vec<Vec<(Alignment, Vec<Token<'a>>)>>,
	),
	/// TaskBox: Boolean state of the checked or unchecked box. str: List item
	/// text
	TaskListItem(TaskBox, String),
	/// First str: Reference id. Second str: Reference text
	Footnote(String, String),
}

impl fmt::Display for Token<'_> {
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

impl Token<'_> {
	pub(crate) fn is_usable_in_table(&self) -> bool {
		match self {
			Token::Code(_) => true,
			Token::Link(_, _, _) => true,
			Token::Bold(_) => true,
			Token::Italic(_) => true,
			Token::BoldItalic(_) => true,
			Token::Plaintext(_) => true,
			_ => false,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValidURL<'a> {
	pub(crate) content: &'a str,
	pub(crate) scheme:  Option<Scheme<'a>>,
}

impl ValidURL<'_> {
	pub(crate) fn fmt_unsafe(&self) -> String {
		let amp_replace_content = self.content.replace('&', "&amp;");
		match &self.scheme {
			None => format!("http:{}", amp_replace_content),
			Some(Scheme::Email(_s)) => amp_replace_content.to_string(),
			Some(s) => format!("{}:{}", s, amp_replace_content),
		}
	}
}

impl fmt::Display for ValidURL<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self.scheme {
			None => {
				write!(
					f,
					"http:{}",
					percent_encode(self.content).replace('&', "&amp;")
				)
			}
			Some(s) => {
				write!(
					f,
					"{}:{}",
					s,
					percent_encode(self.content).replace('&', "&amp;")
				)
			}
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Scheme<'a> {
	Http(&'a str),
	Email(&'a str),
	Irc(&'a str),
	Other(&'a str),
}

impl fmt::Display for Scheme<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Scheme::Http(s) => write!(f, "{}", s),
			Scheme::Email(s) => write!(f, "{}", s),
			Scheme::Irc(s) => write!(f, "{}", s),
			Scheme::Other(s) => write!(f, "{}", s),
		}
	}
}

pub(crate) fn percent_encode(source: &str) -> String {
	source
		.replace('%', "%25")
		.replace('#', "%23")
		.replace('[', "%5B")
		.replace(']', "%5D")
		.replace('!', "%21")
		.replace('$', "%24")
		.replace("'", "%27")
		.replace('(', "%28")
		.replace(')', "%29")
		.replace('*', "%2A")
		.replace(' ', "%20")
		.replace('\\', "%5C")
}

/// Holds the possible states of a taskbox in a task list
#[derive(Debug, PartialEq, Eq)]
pub enum TaskBox {
	Checked,
	Unchecked,
}

/// Holds the alignment states for the table token
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Alignment {
	Left,
	Right,
	Center,
}

impl fmt::Display for Alignment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Alignment::Left => write!(f, "left"),
			Alignment::Right => write!(f, "right"),
			Alignment::Center => write!(f, "center"),
		}
	}
}

pub(crate) fn validate_link(
	source: &str,
) -> Result<ValidURL, SanitizationError> {
	if !source.is_ascii() || source.contains(char::is_whitespace) {
		// https://www.rfc-editor.org/rfc/rfc3986#section-2
		return Err(SanitizationError { content: source });
	}
	let (scheme, path) = source.split_at(source.find(':').unwrap_or(0));
	if scheme.to_lowercase() == "javascript" || !scheme.is_ascii() {
		return Err(SanitizationError { content: source });
	}
	if scheme.to_lowercase() == "data" && !path.starts_with(":image/") {
		return Err(SanitizationError { content: source });
	}
	if scheme.len() != 0 && (scheme.len() < 2 || scheme.len() > 32) {
		return Err(SanitizationError { content: source });
	}

	// Scheme defined here https://spec.commonmark.org/0.30/#scheme
	// char set in COMMONMARK_SCHEME_ASCII. 2 to 32 chars followed by `:`
	let source_scheme = {
		let parts: Vec<_> = source.split(":").collect();
		if source.contains(':') &&
			parts[0]
				.chars()
				.all(|c| COMMONMARK_SCHEME_ASCII.contains(&c)) &&
			parts[0].len() >= 2 &&
			parts[0].len() <= 32
		{
			match parts[0] {
				"http" => Some(Scheme::Http(parts[0])),
				"mailto" => Some(Scheme::Email(parts[0])),
				"irc" => Some(Scheme::Irc(parts[0])),
				_ => Some(Scheme::Other(parts[0])),
			}
		} else {
			None
		}
	};

	//Check for mail links
	if source.contains('@') &&
		source.matches('@').count() == 1 &&
		!source.contains('\\')
	{
		if source_scheme.is_some() {
			return Ok(ValidURL {
				scheme:  Some(source_scheme.unwrap_or(Scheme::Email("mailto"))),
				content: &source.split(":").last().unwrap(),
			});
		}
		return Ok(ValidURL {
			scheme:  Some(source_scheme.unwrap_or(Scheme::Email("mailto"))),
			content: &source,
		});
	}
	if source.contains('@') &&
		source.matches('@').count() == 1 &&
		source.contains('\\')
	{
		return Err(SanitizationError { content: source });
	}

	match source_scheme {
		Some(Scheme::Http(s)) => Ok(ValidURL {
			content: source
				.strip_prefix(s)
				.unwrap_or("")
				.strip_prefix(":")
				.unwrap_or(""),
			scheme:  Some(Scheme::Http(s)),
		}),
		Some(Scheme::Email(s)) => Ok(ValidURL {
			content: source
				.strip_prefix(s)
				.unwrap_or("")
				.strip_prefix(":")
				.unwrap_or(""),
			scheme:  Some(Scheme::Email(s)),
		}),
		Some(Scheme::Irc(s)) => Ok(ValidURL {
			content: source
				.strip_prefix(s)
				.unwrap_or("")
				.strip_prefix(":")
				.unwrap_or(""),
			scheme:  Some(Scheme::Irc(s)),
		}),
		Some(Scheme::Other(s)) => Ok(ValidURL {
			content: source
				.strip_prefix(s)
				.unwrap_or("")
				.strip_prefix(":")
				.unwrap_or(""),
			scheme:  Some(Scheme::Other(s)),
		}),
		None => Ok(ValidURL {
			content: source,
			scheme:  None,
		}),
	}
}

#[derive(Debug)]
pub(crate) struct SanitizationError<'a> {
	pub(crate) content: &'a str,
}
