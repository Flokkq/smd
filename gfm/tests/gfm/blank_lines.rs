use gfm::Parser;

#[test]
fn gfm_test_197_blank_lines_ignored() {
	let test_html = Parser::render("  \n\naaa\n  \n\n# aaa\n\n  \n");
	let reference_html = "<p>aaa</p>\n<h1>aaa</h1>\n";
	assert_eq!(test_html, reference_html);
}
