use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_338_simple_code_span() {
    let test_html = Parser::render("`foo`");
    let reference_html = "<p><code>foo</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_339_code_span_with_backticks() {
    let test_html = Parser::render("`` foo ` bar ``");
    let reference_html = "<p><code>foo ` bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_340_motivation_for_stripping_spaces() {
    let test_html = Parser::render("` `` `");
    let reference_html = "<p><code>``</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_341_single_space_stripped() {
    let test_html = Parser::render("`  ``  `");
    let reference_html = "<p><code> `` </code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_342_no_stripping_single_side_space() {
    let test_html = Parser::render("` a`");
    let reference_html = "<p><code> a</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_343_no_stripping_unicode_space() {
    let test_html = Parser::render("` b `");
    let reference_html = "<p><code> b </code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_344_code_span_only_spaces() {
    let test_html = Parser::render("` `\n`  `");
    let reference_html = "<p><code> </code>\n<code>  </code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_345_line_endings_to_spaces() {
    let test_html = Parser::render("``\nfoo\nbar  \nbaz\n``");
    let reference_html = "<p><code>foo bar   baz</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_346_preserve_space_after_line_ending() {
    let test_html = Parser::render("``\nfoo \n``");
    let reference_html = "<p><code>foo </code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_347_interior_spaces_preserved() {
    let test_html = Parser::render("`foo   bar \nbaz`");
    let reference_html = "<p><code>foo   bar  baz</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_348_literal_backslashes() {
    let test_html = Parser::render("`foo\\`bar`");
    let reference_html = "<p><code>foo\\</code>bar`</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_349_multiple_backticks() {
    let test_html = Parser::render("``foo`bar``");
    let reference_html = "<p><code>foo`bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_350_multiple_backticks_with_spaces() {
    let test_html = Parser::render("` foo `` bar `");
    let reference_html = "<p><code>foo `` bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_351_code_span_precedence_emphasis() {
    let test_html = Parser::render("*foo`*`");
    let reference_html = "<p>*foo<code>*</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_352_code_span_precedence_links() {
    let test_html = Parser::render("[not a `link](/foo`)");
    let reference_html = "<p>[not a <code>link](/foo</code>)</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_353_code_span_vs_html_tag() {
    let test_html = Parser::render("`<a href=\"`\">`");
    let reference_html = "<p><code>&lt;a href=&quot;</code>&quot;&gt;`</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_354_html_tag_over_code_span() {
    let test_html = Parser::render("<a href=\"`\">`");
    let reference_html = "<p><a href=\"`\">`</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_355_code_span_with_mixed_symbols() {
    let test_html = Parser::render("`<http://foo.bar.`baz>`");
    let reference_html = "<p><code>&lt;http://foo.bar.</code>baz&gt;`</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_356_autolink_with_backticks() {
    let test_html = Parser::render("<http://foo.bar.`baz>`");
    let reference_html =
        "<p><a href=\"http://foo.bar.%60baz\">http://foo.bar.`baz</a>`</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_357_unmatched_backticks_literal() {
    let test_html = Parser::render("```foo``");
    let reference_html = "<p>```foo``</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_358_unclosed_backticks() {
    let test_html = Parser::render("`foo");
    let reference_html = "<p>`foo</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_359_mismatched_backtick_lengths() {
    let test_html = Parser::render("`foo``bar``");
    let reference_html = "<p>`foo<code>bar</code></p>\n";
    assert_eq!(test_html, reference_html);
}
