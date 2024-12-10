use smd_core::gfm::Parser;

#[test]
fn gfm_test_603_simple_autolink() {
	let test_html = Parser::render("<http://foo.bar.baz>");
	let reference_html =
		"<p><a href=\"http://foo.bar.baz\">http://foo.bar.baz</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_604_autolink_with_query() {
	let test_html =
		Parser::render("<http://foo.bar.baz/test?q=hello&id=22&boolean>");
	let reference_html = "<p><a href=\"http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean\">http://foo.bar.baz/test?q=hello&amp;id=22&amp;boolean</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_605_autolink_with_port() {
	let test_html = Parser::render("<irc://foo.bar:2233/baz>");
	let reference_html = "<p><a href=\"irc://foo.bar:2233/baz\">irc://foo.bar:\
	                      2233/baz</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_606_uppercase_autolink() {
	let test_html = Parser::render("<MAILTO:FOO@BAR.BAZ>");
	let reference_html =
		"<p><a href=\"MAILTO:FOO@BAR.BAZ\">MAILTO:FOO@BAR.BAZ</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_607_made_up_scheme_autolink() {
	let test_html = Parser::render("<a+b+c:d>");
	let reference_html = "<p><a href=\"a+b+c:d\">a+b+c:d</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_608_made_up_scheme_with_comma() {
	let test_html = Parser::render("<made-up-scheme://foo,bar>");
	let reference_html = "<p><a href=\"made-up-scheme://foo,bar\"\
	                      >made-up-scheme://foo,bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_609_relative_autolink() {
	let test_html = Parser::render("<http://../>");
	let reference_html = "<p><a href=\"http://../\">http://../</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_610_localhost_autolink() {
	let test_html = Parser::render("<localhost:5001/foo>");
	let reference_html =
		"<p><a href=\"localhost:5001/foo\">localhost:5001/foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_611_invalid_autolink_with_space() {
	let test_html = Parser::render("<http://foo.bar/baz bim>");
	let reference_html = "<p>&lt;http://foo.bar/baz bim&gt;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_612_invalid_backslash_autolink() {
	let test_html = Parser::render("<http://example.com/\\[\\>");
	let reference_html =
        "<p><a href=\"http://example.com/%5C%5B%5C\">http://example.com/\\[\\</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_613_email_autolink() {
	let test_html = Parser::render("<foo@bar.example.com>");
	let reference_html = "<p><a href=\"mailto:foo@bar.example.com\">foo@bar.\
	                      example.com</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_614_email_autolink_with_plus() {
	let test_html = Parser::render("<foo+special@Bar.baz-bar0.com>");
	let reference_html = "<p><a href=\"mailto:foo+special@Bar.baz-bar0.com\"\
	                      >foo+special@Bar.baz-bar0.com</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_615_invalid_email_with_backslash() {
	let test_html = Parser::render("<foo\\+@bar.example.com>");
	let reference_html = "<p>&lt;foo+@bar.example.com&gt;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_616_empty_autolink() {
	let test_html = Parser::render("<>");
	let reference_html = "<p>&lt;&gt;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_617_autolink_with_spaces() {
	let test_html = Parser::render("< http://foo.bar >");
	let reference_html = "<p>&lt; http://foo.bar &gt;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_618_invalid_scheme() {
	let test_html = Parser::render("<m:abc>");
	let reference_html = "<p>&lt;m:abc&gt;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_619_non_uri_autolink() {
	let test_html = Parser::render("<foo.bar.baz>");
	let reference_html = "<p>&lt;foo.bar.baz&gt;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_620_non_autolink_uri() {
	let test_html = Parser::render("http://example.com");
	let reference_html = "<p>http://example.com</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_621_non_autolink_email() {
	let test_html = Parser::render("foo@bar.example.com");
	let reference_html = "<p>foo@bar.example.com</p>\n";
	assert_eq!(test_html, reference_html);
}
