use smd_core::gfm::Parser;

#[test]
fn gfm_test_32_simple_atx_headings() {
	let test_html = Parser::render(
		"# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo",
	);
	let expected_html = "<h1>foo</h1>\n<h2>foo</h2>\n<h3>foo</h3>\n<h4>foo</\
	                     h4>\n<h5>foo</h5>\n<h6>foo</h6>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_33_atx_heading_too_many_hashes() {
	let test_html = Parser::render("####### foo");
	let expected_html = "<p>####### foo</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_34_atx_heading_missing_space() {
	let test_html = Parser::render("#5 bolt\n\n#hashtag");
	let expected_html = "<p>#5 bolt</p>\n<p>#hashtag</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_35_atx_heading_escaped_hash() {
	let test_html = Parser::render("\\## foo");
	let expected_html = "<p>## foo</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_36_atx_heading_inline_content() {
	let test_html = Parser::render("# foo *bar* \\*baz\\*");
	let expected_html = "<h1>foo <em>bar</em> *baz*</h1>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_37_atx_heading_trailing_whitespace() {
	let test_html =
		Parser::render("#                  foo                      ");
	let expected_html = "<h1>foo</h1>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_38_atx_heading_indentation_allowed() {
	let test_html = Parser::render(" ### foo\n  ## foo\n   # foo");
	let expected_html = "<h3>foo</h3>\n<h2>foo</h2>\n<h1>foo</h1>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_39_atx_heading_indentation_too_much() {
	let test_html = Parser::render("    # foo");
	let expected_html = "<pre><code># foo\n</code></pre>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_40_atx_heading_in_paragraph() {
	let test_html = Parser::render("foo\n    # bar");
	let expected_html = "<p>foo\n# bar</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_41_atx_heading_closing_hash_sequence() {
	let test_html = Parser::render("## foo ##\n  ###   bar    ###");
	let expected_html = "<h2>foo</h2>\n<h3>bar</h3>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_42_atx_heading_closing_hash_sequence_length() {
	let test_html = Parser::render(
		"# foo ##################################\n##### foo ##",
	);
	let expected_html = "<h1>foo</h1>\n<h5>foo</h5>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_43_atx_heading_closing_hash_with_spaces() {
	let test_html = Parser::render("### foo ###     ");
	let expected_html = "<h3>foo</h3>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_44_atx_heading_closing_hash_with_text() {
	let test_html = Parser::render("### foo ### b");
	let expected_html = "<h3>foo ### b</h3>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_45_atx_heading_closing_hash_without_space() {
	let test_html = Parser::render("# foo#");
	let expected_html = "<h1>foo#</h1>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_46_atx_heading_escaped_closing_hash() {
	let test_html = Parser::render("### foo \\###\n## foo #\\##\n# foo \\#");
	let expected_html = "<h3>foo ###</h3>\n<h2>foo ###</h2>\n<h1>foo #</h1>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_47_atx_heading_interrupted_by_hr() {
	let test_html = Parser::render("****\n## foo\n****");
	let expected_html = "<hr />\n<h2>foo</h2>\n<hr />\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_48_atx_heading_interrupted_paragraph() {
	let test_html = Parser::render("Foo bar\n# baz\nBar foo");
	let expected_html = "<p>Foo bar</p>\n<h1>baz</h1>\n<p>Bar foo</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_49_atx_heading_empty() {
	let test_html = Parser::render("## \n#\n### ###");
	let expected_html = "<h2></h2>\n<h1></h1>\n<h3></h3>\n";
	assert_eq!(test_html, expected_html);
}
