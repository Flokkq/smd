use core::fmt;

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
	fn is_usable_in_table(&self) -> bool {
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
	content: &'a str,
	scheme:  Option<Scheme<'a>>,
}

impl ValidURL<'_> {
	fn fmt_unsafe(&self) -> String {
		let amp_replace_content = self.content.replace('&', "&amp;");
		match &self.scheme {
			None => format!("http:{}", amp_replace_content),
			Some(Scheme::Email(_s)) => {
				amp_replace_content.to_string()
			}
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
