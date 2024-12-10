use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_189_two_simple_paragraphs() {
	let test_html = Parser::render("aaa\n\nbbb\n");
	let reference_html = "<p>aaa</p>\n<p>bbb</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_190_paragraphs_with_multiple_lines() {
	let test_html = Parser::render("aaa\nbbb\n\nccc\nddd\n");
	let reference_html = "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_191_multiple_blank_lines_between_paragraphs() {
	let test_html = Parser::render("aaa\n\n\nbbb\n");
	let reference_html = "<p>aaa</p>\n<p>bbb</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_192_paragraph_with_leading_spaces() {
	let test_html = Parser::render("  aaa\n bbb\n");
	let reference_html = "<p>aaa\nbbb</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_193_paragraph_with_indented_lines_after_first() {
	let test_html = Parser::render(
		"aaa\n             bbb\n                                       ccc\n",
	);
	let reference_html = "<p>aaa\nbbb\nccc</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_194_first_line_indented_up_to_three_spaces() {
	let test_html = Parser::render("   aaa\nbbb\n");
	let reference_html = "<p>aaa\nbbb</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_195_first_line_indented_four_spaces_triggers_code_block() {
	let test_html = Parser::render("    aaa\nbbb\n");
	let reference_html = "<pre><code>aaa\n</code></pre>\n<p>bbb</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_196_paragraph_with_final_spaces() {
	let test_html = Parser::render("aaa     \nbbb     \n");
	let reference_html = "<p>aaa<br />\nbbb</p>\n";
	assert_eq!(test_html, reference_html);
}
