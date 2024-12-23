use gfm::Parser;

#[test]
fn gfm_test_textual_content_675() {
	let test_html = Parser::render("hello $.;'there");
	let expected_html = "<p>hello $.;'there</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_textual_content_676() {
	let test_html = Parser::render("Foo χρῆν");
	let expected_html = "<p>Foo χρῆν</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_textual_content_677() {
	let test_html = Parser::render("Multiple     spaces");
	let expected_html = "<p>Multiple     spaces</p>\n";
	assert_eq!(test_html, expected_html);
}
