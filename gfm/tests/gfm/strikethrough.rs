use gfm::Parser;

#[test]
fn gfm_test_491_simple_strikethrough() {
	let test_html = Parser::render("~~Hi~~ Hello, ~there~ world!");
	let reference_html =
		"<p><del>Hi</del> Hello, <del>there</del> world!</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_492_strikethrough_with_new_paragraph() {
	let test_html = Parser::render("This ~~has a\n\nnew paragraph~~.");
	let reference_html = "<p>This ~~has a</p>\n<p>new paragraph~~.</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_493_no_strikethrough_with_three_tildes() {
	let test_html = Parser::render("This will ~~~not~~~ strike.");
	let reference_html = "<p>This will ~~~not~~~ strike.</p>\n";
	assert_eq!(test_html, reference_html);
}
