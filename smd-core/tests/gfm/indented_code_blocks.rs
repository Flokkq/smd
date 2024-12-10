use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_77_simple_indented_code_block() {
	let test_html = Parser::render("    a simple\n      indented code block\n");
	let reference_html =
		"<pre><code>a simple\n  indented code block\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_78_code_block_vs_list_item_ambiguity() {
	let test_html = Parser::render("  - foo\n\n    bar\n");
	let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_79_nested_list_with_indented_code() {
	let test_html = Parser::render("1.  foo\n\n    - bar\n");
	let reference_html =
		"<ol>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_80_literal_text_in_code_block() {
	let test_html = Parser::render("    <a/>\n    *hi*\n\n    - one\n");
	let reference_html =
		"<pre><code>&lt;a/&gt;\n*hi*\n\n- one\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_81_multiple_chunks_in_code_block() {
	let test_html =
		Parser::render("    chunk1\n\n    chunk2\n  \n \n \n    chunk3\n");
	let reference_html =
		"<pre><code>chunk1\n\nchunk2\n\n\n\nchunk3\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_82_extra_indentation_in_code_block() {
	let test_html = Parser::render("    chunk1\n      \n      chunk2\n");
	let reference_html = "<pre><code>chunk1\n  \n  chunk2\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_83_code_block_does_not_interrupt_paragraph() {
	let test_html = Parser::render("Foo\n    bar\n");
	let reference_html = "<p>Foo\nbar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_84_code_block_and_paragraph() {
	let test_html = Parser::render("    foo\nbar\n");
	let reference_html = "<pre><code>foo\n</code></pre>\n<p>bar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_85_code_block_before_and_after_other_blocks() {
	let test_html =
		Parser::render("# Heading\n    foo\nHeading\n------\n    foo\n----\n");
	let reference_html = "<h1>Heading</h1>\n<pre><code>foo\n</code></pre>\\
	                      n<h2>Heading</h2>\n<pre><code>foo\n</code></pre>\\
	                      n<hr />\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_86_first_line_more_than_four_spaces() {
	let test_html = Parser::render("        foo\n    bar\n");
	let reference_html = "<pre><code>    foo\nbar\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_87_blank_lines_around_code_block() {
	let test_html = Parser::render("\n    foo\n    \n");
	let reference_html = "<pre><code>foo\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_88_trailing_spaces_in_code_block() {
	let test_html = Parser::render("    foo  \n");
	let reference_html = "<pre><code>foo  \n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}
