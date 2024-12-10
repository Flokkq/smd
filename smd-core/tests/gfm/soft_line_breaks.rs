use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_soft_line_breaks_673() {
	let test_html = Parser::render("foo\nbaz");
	let expected_html = "<p>foo\nbaz</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_soft_line_breaks_674() {
	let test_html = Parser::render("foo \n baz");
	let expected_html = "<p>foo\nbaz</p>\n";
	assert_eq!(test_html, expected_html);
}
