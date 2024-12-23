use gfm::Parser;

#[test]
fn gfm_test_disallowed_raw_html_657() {
	let test_html = Parser::render(
		"<strong> <title> <style> <em>\n\n<blockquote>\n  <xmp> is \
		 disallowed.  <XMP> is also disallowed.\n</blockquote>",
	);
	let expected_html = "<p><strong> &lt;title> &lt;style> \
	                     <em></p>\n<blockquote>\n  &lt;xmp> is disallowed.  \
	                     &lt;XMP> is also disallowed.\n</blockquote>\n";
	assert_eq!(test_html, expected_html);
}
