use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_13_simple_thematic_breaks() {
	let test_html = Parser::render("***\n---\n___\n");
	let reference_html = "<hr />\n<hr />\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_14_wrong_characters_in_break() {
	let test_html = Parser::render("+++\n");
	let reference_html = "<p>+++</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_15_equals_signs_in_break() {
	let test_html = Parser::render("===\n");
	let reference_html = "<p>===</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_16_not_enough_characters() {
	let test_html = Parser::render("--\n**\n__\n");
	let reference_html = "<p>--\n**\n__</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_17_thematic_break_with_indentation() {
	let test_html = Parser::render(" ***\n  ***\n   ***\n");
	let reference_html = "<hr />\n<hr />\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_18_four_spaces_is_not_a_thematic_break() {
	let test_html = Parser::render("    ***\n");
	let reference_html = "<pre><code>***\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_19_thematic_break_follows_paragraph() {
	let test_html = Parser::render("Foo\n    ***\n");
	let reference_html = "<p>Foo\n***</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_20_more_than_three_characters_in_break() {
	let test_html = Parser::render("_____________________________________\n");
	let reference_html = "<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_21_spaces_between_thematic_break_characters() {
	let test_html = Parser::render(" - - -\n");
	let reference_html = "<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_22_complex_thematic_break_with_spaces() {
	let test_html = Parser::render(" **  * ** * ** * **\n");
	let reference_html = "<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_23_multiple_spaces_between_break_characters() {
	let test_html = Parser::render("-     -      -      -\n");
	let reference_html = "<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_24_thematic_break_with_trailing_spaces() {
	let test_html = Parser::render("- - - -    \n");
	let reference_html = "<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_25_invalid_characters_in_thematic_break() {
	let test_html = Parser::render("_ _ _ _ a\n\na------\n\n---a---\n");
	let reference_html = "<p>_ _ _ _ a</p>\n<p>a------</p>\n<p>---a---</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_26_invalid_mixed_characters_in_thematic_break() {
	let test_html = Parser::render(" *-*\n");
	let reference_html = "<p><em>-</em></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_27_thematic_break_in_list() {
	let test_html = Parser::render("- foo\n***\n- bar\n");
	let reference_html =
		"<ul>\n<li>foo</li>\n</ul>\n<hr />\n<ul>\n<li>bar</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_28_thematic_break_interrupting_paragraph() {
	let test_html = Parser::render("Foo\n***\nbar\n");
	let reference_html = "<p>Foo</p>\n<hr />\n<p>bar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_29_thematic_break_as_setext_heading() {
	let test_html = Parser::render("Foo\n---\nbar\n");
	let reference_html = "<h2>Foo</h2>\n<p>bar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_30_thematic_break_and_list_item_conflict() {
	let test_html = Parser::render("* Foo\n* * *\n* Bar\n");
	let reference_html =
		"<ul>\n<li>Foo</li>\n</ul>\n<hr />\n<ul>\n<li>Bar</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_31_thematic_break_in_list_item() {
	let test_html = Parser::render("- Foo\n- * * *\n");
	let reference_html = "<ul>\n<li>Foo</li>\n<li>\n<hr />\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}
