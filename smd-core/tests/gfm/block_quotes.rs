use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_206_simple_block_quote() {
    let test_html = Parser::render("> # Foo\n> bar\n> baz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_207_block_quote_without_spaces_after_marker() {
    let test_html = Parser::render("># Foo\n>bar\n> baz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_208_block_quote_with_indented_marker() {
    let test_html = Parser::render("   > # Foo\n   > bar\n > baz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_209_block_quote_as_code_block() {
    let test_html = Parser::render("    > # Foo\n    > bar\n    > baz\n");
    let reference_html = "<pre><code>&gt; # Foo\n&gt; bar\n&gt; baz\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_210_lazy_block_quote_continuation() {
    let test_html = Parser::render("> # Foo\n> bar\nbaz\n");
    let reference_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_211_lazy_and_non_lazy_block_quote_lines() {
    let test_html = Parser::render("> bar\nbaz\n> foo\n");
    let reference_html = "<blockquote>\n<p>bar\nbaz\nfoo</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_212_block_quote_with_thematic_break() {
    let test_html = Parser::render("> foo\n---\n");
    let reference_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_213_block_quote_with_list_item() {
    let test_html = Parser::render("> - foo\n- bar\n");
    let reference_html =
        "<blockquote>\n<ul>\n<li>foo</li>\n</ul>\n</blockquote>\n<ul>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_214_indented_code_block_in_block_quote() {
    let test_html = Parser::render(">     foo\n    bar\n");
    let reference_html = "<blockquote>\n<pre><code>foo\n</code></pre>\n</blockquote>\n<pre><code>bar\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_215_fenced_code_block_in_block_quote() {
    let test_html = Parser::render("> ```\nfoo\n```\n");
    let reference_html = "<blockquote>\n<pre><code></code></pre>\n</blockquote>\n<p>foo</p>\n<pre><code></code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_216_lazy_continuation_of_list_in_block_quote() {
    let test_html = Parser::render("> foo\n    - bar\n");
    let reference_html = "<blockquote>\n<p>foo\n- bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_217_empty_block_quote() {
    let test_html = Parser::render(">\n");
    let reference_html = "<blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_218_empty_block_quote_with_whitespace() {
    let test_html = Parser::render(">\n>  \n> \n");
    let reference_html = "<blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_219_block_quote_with_blank_lines() {
    let test_html = Parser::render(">\n> foo\n>  \n");
    let reference_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_220_separate_block_quotes_with_blank_line() {
    let test_html = Parser::render("> foo\n\n> bar\n");
    let reference_html =
        "<blockquote>\n<p>foo</p>\n</blockquote>\n<blockquote>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_221_single_block_quote_from_consecutive_lines() {
    let test_html = Parser::render("> foo\n> bar\n");
    let reference_html = "<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_222_block_quote_with_two_paragraphs() {
    let test_html = Parser::render("> foo\n>\n> bar\n");
    let reference_html = "<blockquote>\n<p>foo</p>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_223_block_quote_interrupts_paragraph() {
    let test_html = Parser::render("foo\n> bar\n");
    let reference_html = "<p>foo</p>\n<blockquote>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_224_no_blank_lines_needed_around_block_quote() {
    let test_html = Parser::render("> aaa\n***\n> bbb\n");
    let reference_html = "<blockquote>\n<p>aaa</p>\n</blockquote>\n<hr />\n<blockquote>\n<p>bbb</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_225_lazy_block_quote_continuation() {
    let test_html = Parser::render("> bar\nbaz\n");
    let reference_html = "<blockquote>\n<p>bar\nbaz</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_226_block_quote_with_blank_line_and_paragraph() {
    let test_html = Parser::render("> bar\n\nbaz\n");
    let reference_html = "<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_227_block_quote_with_blank_line_followed_by_text() {
    let test_html = Parser::render("> bar\n>\nbaz\n");
    let reference_html = "<blockquote>\n<p>bar</p>\n</blockquote>\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_228_nested_block_quotes_with_lazy_continuation() {
    let test_html = Parser::render("> > > foo\nbar\n");
    let reference_html = "<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar</p>\n</blockquote>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_229_nested_block_quotes_with_mixed_markers() {
    let test_html = Parser::render(">>> foo\n> bar\n>>baz\n");
    let reference_html = "<blockquote>\n<blockquote>\n<blockquote>\n<p>foo\nbar\nbaz</p>\n</blockquote>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_230_indented_code_block_in_block_quote() {
    let test_html = Parser::render(">     code\n\n>    not code\n");
    let reference_html = "<blockquote>\n<pre><code>code\n</code></pre>\n</blockquote>\n<blockquote>\n<p>not code</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}
