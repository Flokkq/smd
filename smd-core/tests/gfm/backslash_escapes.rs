use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_308_backslash_escape_ascii_punctuation() {
    let test_html = Parser::render("\\!\\\"\\#\\$\\%\\&\\\'\\(\\)\\*\\+\\,\\-\\.\\/\\:\\;\\<\\=\\>\\?\\@\\[\\\\\\]\\^\\_\\`\\{\\|\\}\\~\n");
    let reference_html = "<p>!&quot;#$%&amp;'()*+,-./:;&lt;=&gt;?@[\\]^_`{|}~</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_309_backslash_escape_other_characters() {
    let test_html = Parser::render(r#"\→\A\a\ \3\φ\«"#);
    let reference_html = "<p>\\→\\A\\a\\ \\3\\φ\\«</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_310_escaped_characters_literal() {
    let test_html = Parser::render("\\*not emphasized*\n\\<br/> not a tag\n\\[not a link](/foo)\n\\`not code`\n1\\. not a list\n\\* not a list\n\\# not a heading\n\\[foo]: /url \"not a reference\"\n\\&ouml; not a character entity\n");
    let reference_html = "<p>*not emphasized*\n&lt;br/&gt; not a tag\n[not a link](/foo)\n`not code`\n1. not a list\n* not a list\n# not a heading\n[foo]: /url &quot;not a reference&quot;\n&amp;ouml; not a character entity</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_311_escaped_backslash() {
    let test_html = Parser::render(r#"\\*emphasis*"#);
    let reference_html = "<p>\\<em>emphasis</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_312_backslash_hard_line_break() {
    let test_html = Parser::render("foo\\\nbar");
    let reference_html = "<p>foo<br />\nbar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_313_backslash_in_code_span() {
    let test_html = Parser::render("`` \\[\\` ``");
    let reference_html = "<p><code>\\[\\`</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_314_backslash_in_code_block_indented() {
    let test_html = Parser::render("    \\[\\]\n");
    let reference_html = "<pre><code>\\[\\]\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_315_backslash_in_fenced_code_block() {
    let test_html = Parser::render("~~~\n\\[\\]\n~~~");
    let reference_html = "<pre><code>\\[\\]\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_316_backslash_in_autolink() {
    let test_html = Parser::render("<http://example.com?find=\\*>");
    let reference_html =
        r#"<p><a href="http://example.com?find=%5C*">http://example.com?find=\*</a></p>"#;
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_317_backslash_in_html_tag() {
    let test_html = Parser::render("<a href=\"/bar\\/)\">");
    let reference_html = "<a href=\"/bar\\/)\">\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_318_backslash_in_link_and_title() {
    let test_html = Parser::render(r#"[foo](/bar\* "ti\*tle")"#);
    let reference_html = r#"<p><a href="/bar*" title="ti*tle">foo</a></p>"#;
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_319_backslash_in_link_reference() {
    let test_html = Parser::render("[foo]\n\n[foo]: /bar\\* \"ti\\*tle\"");
    let reference_html = r#"<p><a href="/bar*" title="ti*tle">foo</a></p>"#;
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_320_backslash_in_info_string() {
    let test_html = Parser::render("``` foo\\+bar\nfoo\n```");
    let reference_html = r#"<pre><code class="language-foo+bar">foo
</code></pre>"#;
    assert_eq!(test_html, reference_html);
}
