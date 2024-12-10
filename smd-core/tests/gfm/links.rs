use smd_core::gfm::Parser;

#[test]
fn gfm_test_494_inline_link_with_title() {
	let test_html = Parser::render("[link](/uri \"title\")");
	let reference_html = "<p><a href=\"/uri\" title=\"title\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_495_inline_link_without_title() {
	let test_html = Parser::render("[link](/uri)");
	let reference_html = "<p><a href=\"/uri\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_496_empty_inline_link() {
	let test_html = Parser::render("[link]()");
	let reference_html = "<p><a href=\"\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_497_empty_inline_link_with_pointy_brackets() {
	let test_html = Parser::render("[link](<>)");
	let reference_html = "<p><a href=\"\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_498_invalid_inline_link_with_spaces() {
	let test_html = Parser::render("[link](/my uri)");
	let reference_html = "<p>[link](/my uri)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_499_valid_inline_link_with_spaces_and_pointy_brackets() {
	let test_html = Parser::render("[link](</my uri>)");
	let reference_html = "<p><a href=\"/my%20uri\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_500_inline_link_with_line_break() {
	let test_html = Parser::render("[link](foo\nbar)");
	let reference_html = "<p>[link](foo\nbar)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_501_inline_link_with_line_break_in_pointy_brackets() {
	let test_html = Parser::render("[link](<foo\nbar>)");
	let reference_html = "<p>[link](<foo\nbar>)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_502_inline_link_with_closing_parenthesis_in_pointy_brackets() {
	let test_html = Parser::render("[a](<b)c>)");
	let reference_html = "<p><a href=\"b)c\">a</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_503_unescaped_closing_pointy_bracket_in_link() {
	let test_html = Parser::render("[link](<foo\\>)");
	let reference_html = "<p>[link](&lt;foo&gt;)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_504_unmatched_pointy_brackets_in_link() {
	let test_html = Parser::render("[a](<b)c\n[a](<b)c>\n[a](<b>c)");
	let reference_html = "<p>[a](&lt;b)c\n[a](&lt;b)c&gt;\n[a](<b>c)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_505_escaped_parentheses_in_link() {
	let test_html = Parser::render("[link](\\(foo\\))");
	let reference_html = "<p><a href=\"(foo)\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_506_balanced_parentheses_in_link() {
	let test_html = Parser::render("[link](foo(and(bar)))");
	let reference_html = "<p><a href=\"foo(and(bar))\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_507_unbalanced_parentheses_with_escape() {
	let test_html = Parser::render("[link](foo\\(and\\(bar\\))");
	let reference_html = "<p><a href=\"foo(and(bar)\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_508_unbalanced_parentheses_in_pointy_brackets() {
	let test_html = Parser::render("[link](<foo(and(bar)>)");
	let reference_html = "<p><a href=\"foo(and(bar)\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_509_escaped_characters_in_link() {
	let test_html = Parser::render("[link](foo\\)\\:)");
	let reference_html = "<p><a href=\"foo):\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_510_link_with_fragment_and_query() {
	let test_html = Parser::render("[link](#fragment)\n\n[link](http://example.com#fragment)\n\n[link](http://example.com?foo=3#frag)");
	let reference_html = "<p><a href=\"#fragment\">link</a></p>\n<p><a href=\"http://example.com#fragment\">link</a></p>\n<p><a href=\"http://example.com?foo=3#frag\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_511_backslash_in_link() {
	let test_html = Parser::render("[link](foo\\bar)");
	let reference_html = "<p><a href=\"foo%5Cbar\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_512_entity_and_numeric_references_in_link() {
	let test_html = Parser::render("[link](foo%20b&auml;)");
	let reference_html = "<p><a href=\"foo%20b%C3%A4\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_513_omitted_destination_with_title() {
	let test_html = Parser::render("[link](\"title\")");
	let reference_html = "<p><a href=\"%22title%22\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_514_different_quote_types_in_title() {
	let test_html = Parser::render(
		"[link](/url \"title\")\n[link](/url 'title')\n[link](/url (title))",
	);
	let reference_html = "<p><a href=\"/url\" title=\"title\">link</a>\n<a \
	                      href=\"/url\" title=\"title\">link</a>\n<a \
	                      href=\"/url\" title=\"title\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_515_escaped_characters_in_title() {
	let test_html = Parser::render("[link](/url \"title \\\"&quot;\")");
	let reference_html =
		"<p><a href=\"/url\" title=\"title &quot;&quot;\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_516_unicode_whitespace_in_title() {
	let test_html = Parser::render("[link](/url \"title\")");
	let reference_html = "<p><a href=\"/url%C2%A0%22title%22\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_517_nested_quotes_in_title() {
	let test_html = Parser::render("[link](/url \"title \"and\" title\")");
	let reference_html =
		"<p>[link](/url &quot;title &quot;and&quot; title&quot;)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_518_workaround_nested_quotes() {
	let test_html = Parser::render("[link](/url 'title \"and\" title')");
	let reference_html = "<p><a href=\"/url\" title=\"title &quot;and&quot; \
	                      title\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_519_whitespace_around_destination_and_title() {
	let test_html = Parser::render("[link](   /uri\n  \"title\"  )");
	let reference_html = "<p><a href=\"/uri\" title=\"title\">link</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_520_whitespace_after_link_text() {
	let test_html = Parser::render("[link] (/uri)");
	let reference_html = "<p>[link] (/uri)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_521_balanced_brackets_in_link_text() {
	let test_html = Parser::render("[link [foo [bar]]](/uri)");
	let reference_html = "<p><a href=\"/uri\">link [foo [bar]]</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_522_unbalanced_brackets_in_link_text() {
	let test_html = Parser::render("[link] bar](/uri)");
	let reference_html = "<p>[link] bar](/uri)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_523_unbalanced_brackets_nested() {
	let test_html = Parser::render("[link [bar](/uri)");
	let reference_html = "<p>[link <a href=\"/uri\">bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_524_escaped_unbalanced_brackets_in_link_text() {
	let test_html = Parser::render("[link \\[bar](/uri)");
	let reference_html = "<p><a href=\"/uri\">link [bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_525_inline_content_in_link_text() {
	let test_html = Parser::render("[link *foo **bar** `#`*](/uri)");
	let reference_html = "<p><a href=\"/uri\">link <em>foo \
	                      <strong>bar</strong> <code>#</code></em></a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_526_image_inside_link() {
	let test_html = Parser::render("[![moon](moon.jpg)](/uri)");
	let reference_html =
		"<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_527_nested_links_not_allowed() {
	let test_html = Parser::render("[foo [bar](/uri)](/uri)");
	let reference_html = "<p>[foo <a href=\"/uri\">bar</a>](/uri)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_528_nested_emphasis_and_links() {
	let test_html = Parser::render("[foo *[bar [baz](/uri)](/uri)*](/uri)");
	let reference_html =
		"<p>[foo <em>[bar <a href=\"/uri\">baz</a>](/uri)</em>](/uri)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_529_image_with_nested_links() {
	let test_html = Parser::render("![[[foo](uri1)](uri2)](uri3)");
	let reference_html = "<p><img src=\"uri3\" alt=\"[foo](uri2)\" /></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_530_emphasis_with_link_text() {
	let test_html = Parser::render("*[foo*](/uri)");
	let reference_html = "<p>*<a href=\"/uri\">foo*</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_531_emphasis_and_link_destination() {
	let test_html = Parser::render("[foo *bar](baz*)");
	let reference_html = "<p><a href=\"baz*\">foo *bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_532_emphasis_broken_by_non_link_brackets() {
	let test_html = Parser::render("*foo [bar* baz]");
	let reference_html = "<p><em>foo [bar</em> baz]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_533_html_tag_and_link_grouping() {
	let test_html = Parser::render("[foo <bar attr=\"](baz)\">");
	let reference_html = "<p>[foo <bar attr=\"](baz)\"></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_534_code_and_link_grouping() {
	let test_html = Parser::render("[foo`](/uri)`");
	let reference_html = "<p>[foo<code>](/uri)</code></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_535_autolink_and_link_grouping() {
	let test_html = Parser::render("[foo<http://example.com/?search=](uri)> ");
	let reference_html = "<p>[foo<a href=\"http://example.com/?search=%5D(uri)\">http://example.com/?search=](uri)</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_536_full_reference_link() {
	let test_html = Parser::render("[foo][bar]\n\n[bar]: /url \"title\"");
	let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_537_reference_with_balanced_brackets() {
	let test_html = Parser::render("[link [foo [bar]]][ref]\n\n[ref]: /uri");
	let reference_html = "<p><a href=\"/uri\">link [foo [bar]]</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_538_escaped_unbalanced_brackets_in_reference() {
	let test_html = Parser::render("[link \\[bar][ref]\n\n[ref]: /uri");
	let reference_html = "<p><a href=\"/uri\">link [bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_539_inline_content_in_reference() {
	let test_html =
		Parser::render("[link *foo **bar** `#`*][ref]\n\n[ref]: /uri");
	let reference_html = "<p><a href=\"/uri\">link <em>foo \
	                      <strong>bar</strong> <code>#</code></em></a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_540_image_inside_reference() {
	let test_html = Parser::render("[![moon](moon.jpg)][ref]\n\n[ref]: /uri");
	let reference_html =
		"<p><a href=\"/uri\"><img src=\"moon.jpg\" alt=\"moon\" /></a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_541_links_cannot_contain_links_in_reference() {
	let test_html = Parser::render("[foo [bar](/uri)][ref]\n\n[ref]: /uri");
	let reference_html =
		"<p>[foo <a href=\"/uri\">bar</a>]<a href=\"/uri\">ref</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_542_emphasis_and_links_in_reference() {
	let test_html =
		Parser::render("[foo *bar [baz][ref]*][ref]\n\n[ref]: /uri");
	let reference_html = "<p>[foo <em>bar <a href=\"/uri\">baz</a></em>]<a \
	                      href=\"/uri\">ref</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_543_emphasis_with_reference_grouping() {
	let test_html = Parser::render("*[foo*][ref]\n\n[ref]: /uri");
	let reference_html = "<p>*<a href=\"/uri\">foo*</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_544_emphasis_and_reference_grouping() {
	let test_html = Parser::render("[foo *bar][ref]*\n\n[ref]: /uri");
	let reference_html = "<p><a href=\"/uri\">foo *bar</a>*</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_545_html_and_reference_grouping() {
	let test_html = Parser::render("[foo <bar attr=\"][ref]\">\n\n[ref]: /uri");
	let reference_html = "<p>[foo <bar attr=\"][ref]\"></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_546_code_and_reference_grouping() {
	let test_html = Parser::render("[foo`][ref]`\n\n[ref]: /uri");
	let reference_html = "<p>[foo<code>][ref]</code></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_547_autolink_and_reference_grouping() {
	let test_html = Parser::render(
		"[foo<http://example.com/?search=][ref]>\n\n[ref]: /uri",
	);
	let reference_html = "<p>[foo<a href=\"http://example.com/?search=%5D%5Bref%5D\">http://example.com/?search=][ref]</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_548_case_insensitive_reference_matching() {
	let test_html = Parser::render("[foo][BaR]\n\n[bar]: /url \"title\"");
	let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_549_unicode_case_fold_reference_matching() {
	let test_html = Parser::render("[ẞ]\n\n[SS]: /url");
	let reference_html = "<p><a href=\"/url\">ẞ</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_550_internal_whitespace_normalized_in_reference() {
	let test_html = Parser::render("[Foo\n  bar]: /url\n\n[Baz][Foo bar]");
	let reference_html = "<p><a href=\"/url\">Baz</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_551_no_whitespace_between_text_and_label() {
	let test_html = Parser::render("[foo] [bar]\n\n[bar]: /url \"title\"");
	let reference_html =
		"<p>[foo] <a href=\"/url\" title=\"title\">bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_552_separate_lines_for_reference() {
	let test_html = Parser::render("[foo]\n[bar]\n\n[bar]: /url \"title\"");
	let reference_html =
		"<p>[foo]\n<a href=\"/url\" title=\"title\">bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_553_first_matching_reference_is_used() {
	let test_html =
		Parser::render("[foo]: /url1\n\n[foo]: /url2\n\n[bar][foo]");
	let reference_html = "<p><a href=\"/url1\">bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_554_no_normalization_on_reference_inline_content() {
	let test_html = Parser::render("[bar][foo\\!]\n\n[foo!]: /url");
	let reference_html = "<p>[bar][foo!]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_555_unescaped_brackets_in_labels() {
	let test_html = Parser::render("[foo][ref[]\n\n[ref[]: /uri");
	let reference_html = "<p>[foo][ref[]</p>\n<p>[ref[]: /uri</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_556_unbalanced_brackets_in_labels() {
	let test_html = Parser::render("[foo][ref[bar]]\n\n[ref[bar]]: /uri");
	let reference_html = "<p>[foo][ref[bar]]</p>\n<p>[ref[bar]]: /uri</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_557_nested_brackets_in_labels() {
	let test_html = Parser::render("[[[foo]]]\n\n[[[foo]]]: /url");
	let reference_html = "<p>[[[foo]]]</p>\n<p>[[[foo]]]: /url</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_558_backslash_escape_brackets_in_labels() {
	let test_html = Parser::render("[foo][ref\\[]\n\n[ref\\[]: /uri");
	let reference_html = "<p><a href=\"/uri\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_559_escaped_backslash_in_labels() {
	let test_html = Parser::render("[bar\\\\]: /uri\n\n[bar\\\\]");
	let reference_html = "<p><a href=\"/uri\">bar\\</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_560_empty_reference_label_not_allowed() {
	let test_html = Parser::render("[]\n\n[]: /uri");
	let reference_html = "<p>[]</p>\n<p>[]: /uri</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_561_whitespace_only_reference_label_not_allowed() {
	let test_html = Parser::render("[\n ]\n\n[\n ]: /uri");
	let reference_html = "<p>[\n]</p>\n<p>[\n]: /uri</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_562_collapsed_reference_link() {
	let test_html = Parser::render("[foo][]\n\n[foo]: /url \"title\"");
	let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_563_collapsed_reference_with_inline_content() {
	let test_html =
		Parser::render("[*foo* bar][]\n\n[*foo* bar]: /url \"title\"");
	let reference_html =
		"<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_564_case_insensitive_collapsed_reference_matching() {
	let test_html = Parser::render("[Foo][]\n\n[foo]: /url \"title\"");
	let reference_html = "<p><a href=\"/url\" title=\"title\">Foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_565_no_whitespace_between_reference_and_brackets() {
	let test_html = Parser::render("[foo] \n[]\n\n[foo]: /url \"title\"");
	let reference_html =
		"<p><a href=\"/url\" title=\"title\">foo</a>\n[]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_566_shortcut_reference_link() {
	let test_html = Parser::render("[foo]\n\n[foo]: /url \"title\"");
	let reference_html = "<p><a href=\"/url\" title=\"title\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_567_inline_content_in_shortcut_reference() {
	let test_html =
		Parser::render("[*foo* bar]\n\n[*foo* bar]: /url \"title\"");
	let reference_html =
		"<p><a href=\"/url\" title=\"title\"><em>foo</em> bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_568_unexpected_brackets_in_shortcut_reference() {
	let test_html =
		Parser::render("[[*foo* bar]]\n\n[*foo* bar]: /url \"title\"");
	let reference_html =
		"<p>[<a href=\"/url\" title=\"title\"><em>foo</em> bar</a>]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_569_misnested_shortcut_reference() {
	let test_html = Parser::render("[[bar [foo]\n\n[foo]: /url");
	let reference_html = "<p>[[bar <a href=\"/url\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_570_case_insensitive_shortcut_reference_matching() {
	let test_html = Parser::render("[Foo]\n\n[foo]: /url \"title\"");
	let reference_html = "<p><a href=\"/url\" title=\"title\">Foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_571_preserve_space_after_shortcut_reference() {
	let test_html = Parser::render("[foo] bar\n\n[foo]: /url");
	let reference_html = "<p><a href=\"/url\">foo</a> bar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_572_escape_opening_bracket_in_text() {
	let test_html = Parser::render("\\[foo]\n\n[foo]: /url \"title\"");
	let reference_html = "<p>[foo]</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_573_reference_with_asterisks_in_label() {
	let test_html = Parser::render("[foo*]: /url\n\n*[foo*]");
	let reference_html = "<p>*<a href=\"/url\">foo*</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_574_full_reference_takes_precedence_over_shortcut() {
	let test_html = Parser::render("[foo][bar]\n\n[foo]: /url1\n[bar]: /url2");
	let reference_html = "<p><a href=\"/url2\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_575_collapsed_reference_takes_precedence_over_shortcut() {
	let test_html = Parser::render("[foo][]\n\n[foo]: /url1");
	let reference_html = "<p><a href=\"/url1\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_576_inline_link_takes_precedence_over_reference() {
	let test_html = Parser::render("[foo]()\n\n[foo]: /url1");
	let reference_html = "<p><a href=\"\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_577_inline_text_following_inline_link() {
	let test_html = Parser::render("[foo](not a link)\n\n[foo]: /url1");
	let reference_html = "<p><a href=\"/url1\">foo</a>(not a link)</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_578_shortcut_reference_precedence_in_sequence() {
	let test_html = Parser::render("[foo][bar][baz]\n\n[baz]: /url");
	let reference_html = "<p>[foo]<a href=\"/url\">bar</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_579_full_reference_precedence_in_sequence() {
	let test_html =
		Parser::render("[foo][bar][baz]\n\n[baz]: /url1\n[bar]: /url2");
	let reference_html =
		"<p><a href=\"/url2\">foo</a><a href=\"/url1\">baz</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_580_no_shortcut_reference_due_to_following_label() {
	let test_html =
		Parser::render("[foo][bar][baz]\n\n[baz]: /url1\n[foo]: /url2");
	let reference_html = "<p>[foo][bar][baz]</p>\n";
	assert_eq!(test_html, reference_html);
}
