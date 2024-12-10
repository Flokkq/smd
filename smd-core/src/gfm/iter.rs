//
pub struct MarkdownIter<'a> {
	the_str: &'a str,
	index:   usize,
}

impl<'a> Iterator for MarkdownIter<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		for i in 1..=4 {
			if self.the_str.is_char_boundary(self.index + i) {
				let ret = self.the_str.get(self.index..self.index + i);
				self.index += i;
				return ret;
			}
		}
		None
	}
}

impl<'a> MarkdownIter<'a> {
	pub fn new(source: &'a str) -> MarkdownIter<'a> {
		MarkdownIter {
			the_str: source,
			index:   0,
		}
	}

	fn update_index_to(&mut self, i: usize) {
		self.index = i;
	}

	pub fn peek(&self) -> Option<&'a str> {
		for i in 1..=4 {
			if self.the_str.is_char_boundary(self.index + i) {
				return self.the_str.get(self.index..self.index + i);
			}
		}
		None
	}

	pub fn next_if_eq(&mut self, expected: &'a str) -> Option<&'a str> {
		if self.peek() == Some(expected) {
			return self.next();
		}
		None
	}

	pub fn consume_while_case_holds(
		&mut self,
		func: &dyn Fn(&str) -> bool,
	) -> Option<&'a str> {
		let start_index = self.index;
		while self.peek().is_some() && func(self.peek().unwrap()) {
			self.next();
		}
		self.the_str.get(start_index..self.index)
	}

	pub fn consume_until_tail_is(&mut self, tail: &str) -> Option<&'a str> {
		let start_index = self.index;
		while self.peek().is_some() &&
			!self
				.the_str
				.get(start_index..self.index)
				.unwrap_or(tail)
				.ends_with(tail)
		{
			//unwrap_or(tail) to ensure exit in unforseen situation
			self.next();
		}
		self.the_str.get(start_index..self.index)
	}

	pub fn consume_until_end(&mut self) -> Option<&'a str> {
		let start_index = self.index;
		while self.peek().is_some() {
			self.next();
		}
		self.the_str.get(start_index..self.index)
	}

	pub fn peek_until_end(&self) -> Option<&'a str> {
		self.the_str.get(self.index..=(self.the_str.len() - 1))
	}

	pub fn get_index(&self) -> usize {
		self.index
	}

	pub fn get_substring_from(&self, start: usize) -> Option<&'a str> {
		self.the_str.get(start..self.index)
	}

	pub fn get_substring_ahead(&self, end: usize) -> Option<&'a str> {
		self.the_str.get(self.index..(self.index + end))
	}

	pub fn find_next(&self, pattern: &str) -> Option<usize> {
		self.the_str[self.index..].find(pattern)
	}

	pub fn peek_line_ahead(&self) -> Option<&'a str> {
		match self.find_next("\n") {
			Some(newline_index) => {
				return self
					.the_str
					.get(self.index..=(self.index + newline_index))
			}
			None if self.peek().is_some() => {
				return self.the_str.get(self.index..=(self.the_str.len() - 1))
			}
			_ => None,
		}
	}

	pub fn consume_line_ahead(&mut self) -> Option<&'a str> {
		match self.find_next("\n") {
			Some(newline_index) => {
				let ret =
					self.the_str.get(self.index..=(self.index + newline_index));
				self.update_index_to(self.index + newline_index + 1);
				return ret;
			}
			None if self.peek().is_some() => {
				let ret =
					self.the_str.get(self.index..=(self.the_str.len() - 1));
				self.update_index_to(self.the_str.len());
				return ret;
			}
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn peek_does_not_advance() {
		let some_text = "this is some plaintext";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(Some("t"), some_text_iter.peek());
		assert_eq!(Some("t"), some_text_iter.peek());
		assert_eq!(Some("t"), some_text_iter.next());
	}

	#[test]
	fn peek_does_not_advance_utf() {
		let some_text = "الْأَ";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(Some("ا"), some_text_iter.peek());
		assert_eq!(Some("ا"), some_text_iter.peek());
		assert_eq!(Some("ا"), some_text_iter.next());
	}

	#[test]
	fn modern_standard_arabic_test() {
		let some_text = "الْﺦﷺأَ"; // لْ is a weird character. 2 bytes are valid for the base and two more
						   // add the little circle on top.
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(Some("الْﺦﷺأَ"), some_text_iter.consume_line_ahead());
		assert_eq!(None, some_text_iter.next());
	}

	#[test]
	fn consume_until_end_consumes_full_string() {
		let some_text = "this is some plaintext";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(
			Some("this is some plaintext"),
			some_text_iter.consume_until_end()
		);
		assert_eq!(None, some_text_iter.next());
	}

	#[test]
	fn next_advances_utf_correctly() {
		let some_text = "اa木b겅c€d𐍈e";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(Some("ا"), some_text_iter.next());
		assert_eq!(Some("a"), some_text_iter.peek());
		assert_eq!(Some("a"), some_text_iter.next());
		assert_eq!(Some("木"), some_text_iter.peek());
		assert_eq!(Some("木"), some_text_iter.next());
		assert_eq!(Some("b"), some_text_iter.next());
		assert_eq!(Some("겅"), some_text_iter.peek());
		assert_eq!(Some("겅"), some_text_iter.next());
		assert_eq!(Some("c"), some_text_iter.next());
		assert_eq!(Some("€"), some_text_iter.next());
		assert_eq!(Some("d"), some_text_iter.peek());
		assert_eq!(Some("d"), some_text_iter.next());
		assert_eq!(Some("𐍈"), some_text_iter.next());
		assert_eq!(Some("e"), some_text_iter.next());
	}

	#[test]
	fn test_slashes() {
		let slashes = "¯\\\\\\\\\\¯";
		let mut slash_iter = MarkdownIter::new(&slashes);
		assert_eq!(Some("¯"), slash_iter.peek());
		assert_eq!(Some("¯"), slash_iter.next());
		assert_eq!(Some("\\"), slash_iter.peek());
		assert_eq!(
			Some("\\\\\\\\\\"),
			slash_iter.consume_while_case_holds(&|c| c == "\\")
		);
		assert_eq!(Some("¯"), slash_iter.peek());
		assert_eq!(Some("¯"), slash_iter.next());
		assert_eq!(None, slash_iter.next());
	}

	#[test]
	fn general_iter_test() {
		let some_text = "this is some plaintext";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(Some("t"), some_text_iter.peek());
		assert_eq!(Some("t"), some_text_iter.peek());
		assert_eq!(Some("t"), some_text_iter.next());
		assert_eq!(
			Some("his"),
			some_text_iter.consume_while_case_holds(&|c| c != " ")
		);
		assert_eq!(
			Some(" is some plain"),
			some_text_iter.consume_until_tail_is("plain")
		);
		assert_eq!(Some("text"), some_text_iter.consume_until_end());
		assert_eq!(None, some_text_iter.next());

		let other_text = "jkfsgbkfgbdklfdsbh gkhsdfbg <details> and more chars";
		let mut other_text_iter = MarkdownIter::new(&other_text);
		assert_eq!(
			Some("jkfsgbkfgbdklfdsbh gkhsdfbg <details>"),
			other_text_iter.consume_until_tail_is("<details>")
		);
		assert_eq!(
			Some(" and more chars"),
			other_text_iter.consume_until_end()
		);
		assert_eq!(None, other_text_iter.peek());
	}

	#[test]
	fn consume_peek_line_test() {
		let some_text = "this is some plaintext in a line\nAnd a new line \
		                 with more content";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(
			Some("this is some plaintext in a line\n"),
			some_text_iter.peek_line_ahead()
		);
		assert_eq!(
			Some("this is some plaintext in a line\n"),
			some_text_iter.consume_line_ahead()
		);
		assert_ne!(
			Some("this is some plaintext in a line\n"),
			some_text_iter.peek_line_ahead()
		);
		assert_eq!(
			Some("And a new line with more content"),
			some_text_iter.peek_line_ahead()
		);
		assert_eq!(
			Some("And a new line with more content"),
			some_text_iter.consume_line_ahead()
		);
		assert_eq!(None, some_text_iter.peek_line_ahead());
	}

	#[test]
	fn test_degenerate_newlines() {
		let some_text = "\n\n\n\n\nfoo\n";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(Some("\n"), some_text_iter.peek_line_ahead());
		assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
		assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
		assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
		assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
		assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
		assert_eq!(Some("foo\n"), some_text_iter.consume_line_ahead());
		assert_eq!(None, some_text_iter.consume_line_ahead());
	}

	#[test]
	fn test_mixed_chars() {
		let some_text = "  - foo\n\n\tbar\n";
		let mut some_text_iter = MarkdownIter::new(&some_text);
		assert_eq!(Some("  - foo\n"), some_text_iter.consume_line_ahead());
		assert_eq!(Some("\n"), some_text_iter.consume_line_ahead());
		assert_eq!(Some("\tbar\n"), some_text_iter.consume_line_ahead());
	}
}
