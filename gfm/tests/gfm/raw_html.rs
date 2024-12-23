use gfm::Parser;

#[test]
fn gfm_test_raw_html_636_open_tags() {
	let test_html = Parser::render("<a><bab><c2c>");
	let expected_html = "<p><a><bab><c2c></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_637_empty_elements() {
	let test_html = Parser::render("<a/><b2/>");
	let expected_html = "<p><a/><b2/></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_638_whitespace_in_tags() {
	let test_html = Parser::render("<a  /><b2\ndata=\"foo\" >");
	let expected_html = "<p><a  /><b2\ndata=\"foo\" ></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_639_with_attributes() {
	let test_html = Parser::render(
		"<a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean zoop:33=zoop:33 />",
	);
	let expected_html = "<p><a foo=\"bar\" bam = 'baz <em>\"</em>'\n_boolean \
	                     zoop:33=zoop:33 /></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_640_custom_tag_names() {
	let test_html = Parser::render("Foo <responsive-image src=\"foo.jpg\" />");
	let expected_html = "<p>Foo <responsive-image src=\"foo.jpg\" /></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_641_illegal_tag_names() {
	let test_html = Parser::render("<33> <__>");
	let expected_html = "<p>&lt;33&gt; &lt;__&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_642_illegal_attribute_names() {
	let test_html = Parser::render("<a h*#ref=\"hi\">");
	let expected_html = "<p>&lt;a h*#ref=&quot;hi&quot;&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_643_illegal_attribute_values() {
	let test_html = Parser::render("<a href=\"hi'> <a href=hi'>");
	let expected_html = "<p>&lt;a href=&quot;hi'&gt; &lt;a href=hi'&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_644_illegal_whitespace() {
	let test_html =
		Parser::render("< a><\nfoo><bar/ >\n<foo bar=baz\nbim!bop />");
	let expected_html = "<p>&lt; a&gt;&lt;\nfoo&gt;&lt;bar/ &gt;\n&lt;foo \
	                     bar=baz\nbim!bop /&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_645_missing_whitespace() {
	let test_html = Parser::render("<a href='bar'title=title>");
	let expected_html = "<p>&lt;a href='bar'title=title&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_646_closing_tags() {
	let test_html = Parser::render("</a></foo >");
	let expected_html = "<p></a></foo ></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_647_illegal_closing_tag_attributes() {
	let test_html = Parser::render("</a href=\"foo\">");
	let expected_html = "<p>&lt;/a href=&quot;foo&quot;&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_648_valid_comment() {
	let test_html =
		Parser::render("foo <!-- this is a\ncomment - with hyphen -->");
	let expected_html =
		"<p>foo <!-- this is a\ncomment - with hyphen --></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_649_invalid_comment() {
	let test_html = Parser::render("foo <!-- not a comment -- two hyphens -->");
	let expected_html =
		"<p>foo &lt;!-- not a comment -- two hyphens --&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_650_not_a_comment() {
	let test_html = Parser::render("foo <!--> foo -->\n\nfoo <!-- foo--->");
	let expected_html =
		"<p>foo &lt;!--&gt; foo --&gt;</p>\n<p>foo &lt;!-- foo---&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_651_processing_instructions() {
	let test_html = Parser::render("foo <?php echo $a; ?>");
	let expected_html = "<p>foo <?php echo $a; ?></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_652_declarations() {
	let test_html = Parser::render("foo <!ELEMENT br EMPTY>");
	let expected_html = "<p>foo <!ELEMENT br EMPTY></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_653_cdata_sections() {
	let test_html = Parser::render("foo <![CDATA[>&<]]>");
	let expected_html = "<p>foo <![CDATA[>&<]]></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_654_entity_references() {
	let test_html = Parser::render("foo <a href=\"&ouml;\">");
	let expected_html = "<p>foo <a href=\"&ouml;\"></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_655_backslash_escapes() {
	let test_html = Parser::render("foo <a href=\"\\*\">");
	let expected_html = "<p>foo <a href=\"\\*\"></p>\n";
	assert_eq!(test_html, expected_html);
}

#[test]
fn gfm_test_raw_html_656_backslash_in_quotes() {
	let test_html = Parser::render("<a href=\"\\\"\">");
	let expected_html = "<p>&lt;a href=&quot;&quot;&quot;&gt;</p>\n";
	assert_eq!(test_html, expected_html);
}
