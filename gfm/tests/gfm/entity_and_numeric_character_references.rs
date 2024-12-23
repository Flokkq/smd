use gfm::Parser;

#[test]
fn gfm_test_321_html_named_entities() {
	let test_html = Parser::render(
		"&nbsp; &amp; &copy; &AElig; &Dcaron;\n&frac34; &HilbertSpace; \
		 &DifferentialD;\n&ClockwiseContourIntegral; &ngE;",
	);
	let reference_html = "<p>  &amp; © Æ Ď\n¾ ℋ ⅆ\n∲ ≧̸</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_322_decimal_numeric_character_references() {
	let test_html = Parser::render("&#35; &#1234; &#992; &#0;");
	let reference_html = "<p># Ӓ Ϡ �</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_323_hexadecimal_numeric_character_references() {
	let test_html = Parser::render("&#X22; &#XD06; &#xcab;");
	let reference_html = "<p>&quot; ആ ಫ</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_324_invalid_and_nonentities() {
	let test_html = Parser::render(
		"&nbsp &x; &#; &#x;\n&#87654321;\n&#abcdef0;\n&ThisIsNotDefined; &hi?;",
	);
	let reference_html = "<p>&amp;nbsp &amp;x; &amp;#; \
	                      &amp;#x;\n&amp;#87654321;\n&amp;#abcdef0;\n&amp;\
	                      ThisIsNotDefined; &amp;hi?;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_325_missing_semicolon() {
	let test_html = Parser::render("&copy");
	let reference_html = "<p>&amp;copy</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_326_made_up_entity() {
	let test_html = Parser::render("&MadeUpEntity;");
	let reference_html = "<p>&amp;MadeUpEntity;</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_327_entities_in_url() {
	let test_html = Parser::render("<a href=\"&ouml;&ouml;.html\">");
	let reference_html = "<a href=\"&ouml;&ouml;.html\">\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_328_entities_in_link_title() {
	let test_html = Parser::render("[foo](/f&ouml;&ouml; \"f&ouml;&ouml;\")");
	let reference_html =
		"<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_329_entities_in_link_reference() {
	let test_html =
		Parser::render("[foo]\n\n[foo]: /f&ouml;&ouml; \"f&ouml;&ouml;\"");
	let reference_html =
		"<p><a href=\"/f%C3%B6%C3%B6\" title=\"föö\">foo</a></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_330_entities_in_info_strings() {
	let test_html = Parser::render("``` f&ouml;&ouml;\nfoo\n```");
	let reference_html =
		"<pre><code class=\"language-föö\">foo\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_331_entities_in_code_span() {
	let test_html = Parser::render("`f&ouml;&ouml;`");
	let reference_html = "<p><code>f&amp;ouml;&amp;ouml;</code></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_332_entities_in_code_block() {
	let test_html = Parser::render("    f&ouml;f&ouml;\n");
	let reference_html = "<pre><code>f&amp;ouml;f&amp;ouml;\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_333_entities_as_structure_symbols() {
	let test_html = Parser::render("&#42;foo&#42;\n*foo*");
	let reference_html = "<p>*foo*\n<em>foo</em></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_334_entities_as_list_markers() {
	let test_html = Parser::render("&#42; foo\n\n* foo");
	let reference_html = "<p>* foo</p>\n<ul>\n<li>foo</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_335_entities_as_newline_markers() {
	let test_html = Parser::render("foo&#10;&#10;bar");
	let reference_html = "<p>foo\n\nbar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_336_entities_as_tab() {
	let test_html = Parser::render("&#9;foo");
	let reference_html = "<p>→foo</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_337_entities_in_titles() {
	let test_html = Parser::render("[a](url &quot;tit&quot;)");
	let reference_html = "<p>[a](url &quot;tit&quot;)</p>\n";
	assert_eq!(test_html, reference_html);
}
