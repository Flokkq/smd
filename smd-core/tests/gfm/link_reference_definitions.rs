use smd_core::gfm::Parser;

#[test]
fn gfm_test_161_basic_link_reference_definition() {
	let test_html = Parser::render("[foo]: /url \"title\"\n\n[foo]\n");
	let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_162_indented_link_reference_definition() {
	let test_html = Parser::render(
		"   [foo]: \n      /url  \n           'the title'  \n\n[foo]\n",
	);
	let reference_html =
		"<p><a href=\"/url\" title=\"the title\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_163_complex_link_reference_definition() {
	let test_html = Parser::render(
		"[Foo*bar\\]]:my_(url) 'title (with parens)'\n\n[Foo*bar\\]]\n",
	);
	let reference_html = "<p><a href=\"my_(url)\" title=\"title (with \
	                      parens)\">Foo*bar]</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_164_link_reference_definition_with_angle_brackets() {
	let test_html =
		Parser::render("[Foo bar]:\n<my url>\n'title'\n\n[Foo bar]\n");
	let reference_html =
		"<p><a href=\"my%20url\" title=\"title\">Foo bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_165_multiline_link_reference_title() {
	let test_html =
		Parser::render("[foo]: /url '\ntitle\nline1\nline2\n'\n\n[foo]\n");
	let reference_html =
		"<p><a href=\"/url\" title=\"\ntitle\nline1\nline2\n\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_166_link_reference_with_blank_line_in_title() {
	let test_html =
		Parser::render("[foo]: /url 'title\n\nwith blank line'\n\n[foo]\n");
	let reference_html =
		"<p>[foo]: /url 'title</p>\n<p>with blank line'</p>\n<p>[foo]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_167_link_reference_definition_without_title() {
	let test_html = Parser::render("[foo]:\n/url\n\n[foo]\n");
	let reference_html = "<p><a href=\"/url\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_168_link_reference_definition_missing_destination() {
	let test_html = Parser::render("[foo]:\n\n[foo]\n");
	let reference_html = "<p>[foo]:</p>\n<p>[foo]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_169_empty_link_destination() {
	let test_html = Parser::render("[foo]: <>\n\n[foo]\n");
	let reference_html = "<p><a href=\"\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_170_invalid_title_without_whitespace_separator() {
	let test_html = Parser::render("[foo]: <bar>(baz)\n\n[foo]\n");
	let reference_html = "<p>[foo]: <bar>(baz)</p>\n<p>[foo]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_171_backslash_escapes_in_link_reference() {
	let test_html =
		Parser::render("[foo]: /url\\bar\\*baz \"foo\\\"bar\\baz\"\n\n[foo]\n");
	let reference_html = "<p><a href=\"/url%5Cbar*baz\" \
	                      title=\"foo&quot;bar\\baz\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_172_link_before_definition() {
	let test_html = Parser::render("[foo]\n\n[foo]: url\n");
	let reference_html = "<p><a href=\"url\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_173_first_matching_link_reference_takes_precedence() {
	let test_html = Parser::render("[foo]\n\n[foo]: first\n[foo]: second\n");
	let reference_html = "<p><a href=\"first\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_174_case_insensitive_label_matching() {
	let test_html = Parser::render("[FOO]: /url\n\n[Foo]\n");
	let reference_html = "<p><a href=\"/url\">Foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_175_unicode_case_insensitive_label_matching() {
	let test_html = Parser::render("[ΑΓΩ]: /φου\n\n[αγω]\n");
	let reference_html = "<p><a href=\"/%CF%86%CE%BF%CF%85\">αγω</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_176_unused_link_reference_definition() {
	let test_html = Parser::render("[foo]: /url\n");
	let reference_html = "";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_177_multiline_link_reference_label() {
	let test_html = Parser::render("[\nfoo\n]: /url\nbar\n");
	let reference_html = "<p>bar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_178_invalid_link_reference_with_extra_characters() {
	let test_html = Parser::render("[foo]: /url \"title\" ok\n");
	let reference_html = "<p>[foo]: /url &quot;title&quot; ok</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_179_link_reference_without_title() {
	let test_html = Parser::render("[foo]: /url\n\"title\" ok\n");
	let reference_html = "<p>&quot;title&quot; ok</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_180_indented_link_reference_definition() {
	let test_html = Parser::render("    [foo]: /url \"title\"\n\n[foo]\n");
	let reference_html = "<pre><code>[foo]: /url \
	                      &quot;title&quot;\n</code></pre>\n<p>[foo]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_181_link_reference_inside_code_block() {
	let test_html = Parser::render("```\n[foo]: /url\n```\n\n[foo]\n");
	let reference_html =
		"<pre><code>[foo]: /url\n</code></pre>\n<p>[foo]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_182_link_reference_cannot_interrupt_paragraph() {
	let test_html = Parser::render("Foo\n[bar]: /baz\n\n[bar]\n");
	let reference_html = "<p>Foo\n[bar]: /baz</p>\n<p>[bar]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_183_link_reference_after_block_elements() {
	let test_html = Parser::render("# [Foo]\n[foo]: /url\n> bar\n");
	let reference_html = "<h1><a href=\"/url\">Foo</a></h1>\n<blockquote>\\
	                      n<p>bar</p>\n</blockquote>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_184_link_reference_followed_by_setext_heading() {
	let test_html = Parser::render("[foo]: /url\nbar\n===\n[foo]\n");
	let reference_html = "<h1>bar</h1>\n<p><a href=\"/url\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_185_invalid_setext_heading_with_link_reference() {
	let test_html = Parser::render("[foo]: /url\n===\n[foo]\n");
	let reference_html = "<p>===\n<a href=\"/url\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_186_multiple_link_references_without_blank_lines() {
	let test_html = Parser::render(
		"[foo]: /foo-url \"foo\"\n[bar]: /bar-url\n  \"bar\"\n[baz]: \
		 /baz-url\n\n[foo],\n[bar],\n[baz]\n",
	);
	let reference_html = "<p><a href=\"/foo-url\" title=\"foo\">foo</a>,\n<a \
	                      href=\"/bar-url\" title=\"bar\">bar</a>,\n<a \
	                      href=\"/baz-url\">baz</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_187_link_reference_in_blockquote() {
	let test_html = Parser::render("[foo]\n\n> [foo]: /url\n");
	let reference_html =
		"<p><a href=\"/url\">foo</a></p>\n<blockquote>\n</blockquote>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_188_unused_link_reference_definition() {
	let test_html = Parser::render("[foo]: /url\n");
	let reference_html = "";
	assert_eq!(test_html, reference_html);
}
