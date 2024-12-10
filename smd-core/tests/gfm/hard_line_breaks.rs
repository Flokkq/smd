use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_hard_line_breaks_658() {
	let test_html = Parser::render("foo  \nbaz");
	let expected_html = "<p>foo<br />\nbaz</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_659() {
	let test_html = Parser::render("foo\\\nbaz");
	let expected_html = "<p>foo<br />\nbaz</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_660() {
	let test_html = Parser::render("foo       \nbaz");
	let expected_html = "<p>foo<br />\nbaz</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_661() {
	let test_html = Parser::render("foo  \n     bar");
	let expected_html = "<p>foo<br />\nbar</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_662() {
	let test_html = Parser::render("foo\\\n     bar");
	let expected_html = "<p>foo<br />\nbar</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_663() {
	let test_html = Parser::render("*foo  \nbar*");
	let expected_html = "<p><em>foo<br />\nbar</em></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_664() {
	let test_html = Parser::render("*foo\\\nbar*");
	let expected_html = "<p><em>foo<br />\nbar</em></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_665() {
	let test_html = Parser::render("`code  \nspan`");
	let expected_html = "<p><code>code   span</code></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_666() {
	let test_html = Parser::render("`code\\\nspan`");
	let expected_html = "<p><code>code\\ span</code></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_667() {
	let test_html = Parser::render("<a href=\"foo  \nbar\">");
	let expected_html = "<p><a href=\"foo  \nbar\"></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_668() {
	let test_html = Parser::render("<a href=\"foo\\\nbar\">");
	let expected_html = "<p><a href=\"foo\\\nbar\"></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_669() {
	let test_html = Parser::render("foo\\");
	let expected_html = "<p>foo\\</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_670() {
	let test_html = Parser::render("foo  ");
	let expected_html = "<p>foo</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_671() {
	let test_html = Parser::render("### foo\\");
	let expected_html = "<h3>foo\\</h3>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_hard_line_breaks_672() {
	let test_html = Parser::render("### foo  ");
	let expected_html = "<h3>foo</h3>\n";
	assert_eq!(test_html, expected_html);
}
