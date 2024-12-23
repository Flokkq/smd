use gfm::Parser;

#[test]
fn gfm_test_307_inline_code_with_backtick() {
	let test_html = Parser::render("`hi`lo`");
	let reference_html = "<p><code>hi</code>lo`</p>\n";
	assert_eq!(test_html, reference_html);
}
