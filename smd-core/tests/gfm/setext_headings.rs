use smd_core::gfm::Parser;

#[test]
fn gfm_test_50_setext_heading_simple_examples() {
	let test_html =
		Parser::render("Foo *bar*\n=========\n\nFoo *bar*\n---------\n");
	let reference_html =
		"<h1>Foo <em>bar</em></h1>\n<h2>Foo <em>bar</em></h2>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_51_setext_heading_multiline_content() {
	let test_html = Parser::render("Foo *bar\nbaz*\n====\n");
	let reference_html = "<h1>Foo <em>bar\nbaz</em></h1>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_52_setext_heading_leading_and_trailing_whitespace() {
	let test_html = Parser::render("  Foo *bar\nbaz*\t\n====\n");
	let reference_html = "<h1>Foo <em>bar\nbaz</em></h1>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_53_setext_heading_any_length_underlining() {
	let test_html =
		Parser::render("Foo\n-------------------------\n\nFoo\n=\n");
	let reference_html = "<h2>Foo</h2>\n<h1>Foo</h1>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_54_setext_heading_indented_content() {
	let test_html =
		Parser::render("   Foo\n---\n\n  Foo\n-----\n\n  Foo\n  ===\n");
	let reference_html = "<h2>Foo</h2>\n<h2>Foo</h2>\n<h1>Foo</h1>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_55_setext_heading_too_much_indentation() {
	let test_html = Parser::render("    Foo\n    ---\n\n    Foo\n---\n");
	let reference_html = "<pre><code>Foo\n---\n\nFoo\n</code></pre>\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_56_setext_heading_with_indented_underlining() {
	let test_html = Parser::render("Foo\n   ----      \n");
	let reference_html = "<h2>Foo</h2>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_57_setext_heading_invalid_indent_in_underlining() {
	let test_html = Parser::render("Foo\n    ---\n");
	let reference_html = "<p>Foo\n---</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_58_setext_heading_invalid_internal_spaces() {
	let test_html = Parser::render("Foo\n= =\n\nFoo\n--- -\n");
	let reference_html = "<p>Foo\n= =</p>\n<p>Foo</p>\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_59_setext_heading_trailing_spaces_in_content() {
	let test_html = Parser::render("Foo  \n-----\n");
	let reference_html = "<h2>Foo</h2>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_60_setext_heading_with_backslash_at_end() {
	let test_html = Parser::render("Foo\\\n----\n");
	let reference_html = "<h2>Foo\\</h2>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_61_setext_heading_indicators_of_block_structure() {
	let test_html = Parser::render(
		"`Foo\n----\n`\n\n<a title=\"a lot\n---\nof dashes\"/>\n",
	);
	let reference_html = "<h2>`Foo</h2>\n<p>`</p>\n<h2>&lt;a title=&quot;a \
	                      lot</h2>\n<p>of dashes&quot;/&gt;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_62_setext_heading_in_block_quote() {
	let test_html = Parser::render("> Foo\n---\n");
	let reference_html = "<blockquote>\n<p>Foo</p>\n</blockquote>\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_63_setext_heading_with_lazy_continuation() {
	let test_html = Parser::render("> foo\nbar\n===\n");
	let reference_html = "<blockquote>\n<p>foo\nbar\n===</p>\n</blockquote>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_64_setext_heading_in_list_item() {
	let test_html = Parser::render("- Foo\n---\n");
	let reference_html = "<ul>\n<li>Foo</li>\n</ul>\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_65_setext_heading_needs_blank_line_after_paragraph() {
	let test_html = Parser::render("Foo\nBar\n---\n");
	let reference_html = "<h2>Foo\nBar</h2>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_66_setext_heading_no_blank_lines_needed() {
	let test_html = Parser::render("---\nFoo\n---\nBar\n---\nBaz\n");
	let reference_html = "<hr />\n<h2>Foo</h2>\n<h2>Bar</h2>\n<p>Baz</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_67_setext_heading_cannot_be_empty() {
	let test_html = Parser::render("\n====\n");
	let reference_html = "<p>====</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_68_thematic_break_instead_of_setext_heading() {
	let test_html = Parser::render("---\n---\n");
	let reference_html = "<hr />\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_69_setext_heading_after_list() {
	let test_html = Parser::render("- foo\n-----\n");
	let reference_html = "<ul>\n<li>foo</li>\n</ul>\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_70_indented_code_block_instead_of_setext_heading() {
	let test_html = Parser::render("    foo\n---\n");
	let reference_html = "<pre><code>foo\n</code></pre>\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_71_setext_heading_in_block_quote() {
	let test_html = Parser::render("> foo\n-----\n");
	let reference_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_72_setext_heading_literal_text() {
	let test_html = Parser::render("\\> foo\n------\n");
	let reference_html = "<h2>&gt; foo</h2>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_73_setext_heading_with_blank_lines() {
	let test_html = Parser::render("Foo\n\nbar\n---\nbaz\n");
	let reference_html = "<p>Foo</p>\n<h2>bar</h2>\n<p>baz</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_74_setext_heading_with_blank_lines_around_thematic_break() {
	let test_html = Parser::render("Foo\nbar\n\n---\n\nbaz\n");
	let reference_html = "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_75_setext_heading_alternative_thematic_break() {
	let test_html = Parser::render("Foo\nbar\n* * *\nbaz\n");
	let reference_html = "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_76_setext_heading_with_escaped_dashes() {
	let test_html = Parser::render("Foo\nbar\n\\---\nbaz\n");
	let reference_html = "<p>Foo\nbar\n---\nbaz</p>\n";
	assert_eq!(test_html, reference_html);
}
