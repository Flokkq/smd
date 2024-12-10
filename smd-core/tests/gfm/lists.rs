use smd_core::gfm::Parser;

#[test]
fn gfm_test_281_bullet_list_different_markers() {
	let test_html = Parser::render("- foo\n- bar\n+ baz\n");
	let reference_html =
		"<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<ul>\n<li>baz</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_282_ordered_list_different_delimiters() {
	let test_html = Parser::render("1. foo\n2. bar\n3) baz\n");
	let reference_html = "<ol>\n<li>foo</li>\n<li>bar</li>\n</ol>\n<ol \
	                      start=\"3\">\n<li>baz</li>\n</ol>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_283_list_interrupts_paragraph() {
	let test_html = Parser::render("Foo\n- bar\n- baz\n");
	let reference_html =
		"<p>Foo</p>\n<ul>\n<li>bar</li>\n<li>baz</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_284_no_list_in_paragraph() {
	let test_html = Parser::render(
		"The number of windows in my house is\n14.  The number of doors is \
		 6.\n",
	);
	let reference_html = "<p>The number of windows in my house is\n14.  The \
	                      number of doors is 6.</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_285_list_in_paragraph() {
	let test_html = Parser::render(
		"The number of windows in my house is\n1.  The number of doors is 6.\n",
	);
	let reference_html = "<p>The number of windows in my house \
	                      is</p>\n<ol>\n<li>The number of doors is \
	                      6.</li>\n</ol>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_286_blank_lines_between_items() {
	let test_html = Parser::render("- foo\n\n- bar\n\n\n- baz\n");
	let reference_html = "<ul>\n<li>\n<p>foo</p>\n</li>\n<li>\n<p>bar</p>\n</\
	                      li>\n<li>\n<p>baz</p>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_287_nested_lists_with_blank_lines() {
	let test_html =
		Parser::render("- foo\n  - bar\n    - baz\n\n\n      bim\n");
	let reference_html = "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>\n<p>baz</\
	                      p>\n<p>bim</p>\n</li>\n</ul>\n</li>\n</ul>\n</li>\\
	                      n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_288_html_comment_separates_lists() {
	let test_html =
		Parser::render("- foo\n- bar\n\n<!-- -->\n\n- baz\n- bim\n");
	let reference_html = "<ul>\n<li>foo</li>\n<li>bar</li>\n</ul>\n<!-- \
	                      -->\n<ul>\n<li>baz</li>\n<li>bim</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_289_html_comment_prevents_code_block() {
	let test_html = Parser::render(
		"-   foo\n\n    notcode\n\n-   foo\n\n<!-- -->\n\n    code\n",
	);
	let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>notcode</p>\n</li>\n<li>\\
	                      \
	                      n<p>foo</p>\n</li>\n</ul>\n<!-- \
	                      -->\n<pre><code>code\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_290_list_items_not_same_indent_level() {
	let test_html =
		Parser::render("- a\n - b\n  - c\n   - d\n  - e\n - f\n- g\n");
	let reference_html = "<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d</\
	                      li>\n<li>e</li>\n<li>f</li>\n<li>g</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_291_ordered_list_with_varying_indents() {
	let test_html = Parser::render("1. a\n\n  2. b\n\n   3. c\n");
	let reference_html = "<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\\
	                      n<li>\n<p>c</p>\n</li>\n</ol>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_292_paragraph_continuation_line() {
	let test_html = Parser::render("- a\n - b\n  - c\n   - d\n    - e\n");
	let reference_html =
		"<ul>\n<li>a</li>\n<li>b</li>\n<li>c</li>\n<li>d\n- e</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_293_indented_code_in_ordered_list() {
	let test_html = Parser::render("1. a\n\n  2. b\n\n    3. c\n");
	let reference_html = "<ol>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\\
	                      n</ol>\n<pre><code>3. c\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_294_loose_list_with_blank_lines() {
	let test_html = Parser::render("- a\n- b\n\n- c\n");
	let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\\
	                      n<li>\n<p>c</p>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_295_loose_list_with_empty_item() {
	let test_html = Parser::render("* a\n*\n\n* c\n");
	let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li></li>\n<li>\n<p>c</\
	                      p>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_296_loose_list_with_block_elements() {
	let test_html = Parser::render("- a\n- b\n\n  c\n- d\n");
	let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n<p>c</\
	                      p>\n</li>\n<li>\n<p>d</p>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_297_loose_list_with_reference() {
	let test_html = Parser::render("- a\n- b\n\n  [ref]: /url\n- d\n");
	let reference_html = "<ul>\n<li>\n<p>a</p>\n</li>\n<li>\n<p>b</p>\n</li>\\
	                      n<li>\n<p>d</p>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_298_tight_list_with_code_block() {
	let test_html = Parser::render("- a\n- ```\n  b\n\n\n  ```\n- c\n");
	let reference_html = "<ul>\n<li>a</li>\n<li>\n<pre><code>b\n\n\n</code></\
	                      pre>\n</li>\n<li>c</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_299_tight_list_with_loose_sublist() {
	let test_html = Parser::render("- a\n  - b\n\n    c\n- d\n");
	let reference_html = "<ul>\n<li>a\n<ul>\n<li>\n<p>b</p>\n<p>c</p>\n</li>\\
	                      n</ul>\n</li>\n<li>d</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_300_tight_list_with_blockquote() {
	let test_html = Parser::render("* a\n  > b\n  >\n* c\n");
	let reference_html = "<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\\
	                      n</li>\n<li>c</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_301_tight_list_with_nested_code_block() {
	let test_html = Parser::render("- a\n  > b\n  ```\n  c\n  ```\n- d\n");
	let reference_html = "<ul>\n<li>a\n<blockquote>\n<p>b</p>\n</blockquote>\\
	                      n<pre><code>c\n</code></pre>\n</li>\n<li>d</li>\n</\
	                      ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_302_single_paragraph_tight_list() {
	let test_html = Parser::render("- a\n");
	let reference_html = "<ul>\n<li>a</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_303_single_paragraph_nested_tight_list() {
	let test_html = Parser::render("- a\n  - b\n");
	let reference_html = "<ul>\n<li>a\n<ul>\n<li>b</li>\n</ul>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_304_loose_list_with_code_and_paragraph() {
	let test_html = Parser::render("1. ```\n   foo\n   ```\n\n   bar\n");
	let reference_html =
		"<ol>\n<li>\n<pre><code>foo\n</code></pre>\n<p>bar</p>\n</li>\n</ol>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_305_loose_outer_list_tight_inner_list() {
	let test_html = Parser::render("* foo\n  * bar\n\n  baz\n");
	let reference_html = "<ul>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\\
	                      n<p>baz</p>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_306_nested_lists_with_blank_lines() {
	let test_html = Parser::render("- a\n  - b\n  - c\n\n- d\n  - e\n  - f\n");
	let reference_html = "<ul>\n<li>\n<p>a</p>\n<ul>\n<li>b</li>\n<li>c</li>\\
	                      n</ul>\n</li>\n<li>\n<p>d</p>\n<ul>\n<li>e</li>\\
	                      n<li>f</li>\n</ul>\n</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}
