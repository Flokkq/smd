use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_89_fenced_code_block_backticks() {
    let test_html = Parser::render("```\n<\n >\n```\n");
    let reference_html = "<pre><code>&lt;\n &gt;\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_90_fenced_code_block_tildes() {
    let test_html = Parser::render("~~~\n<\n >\n~~~\n");
    let reference_html = "<pre><code>&lt;\n &gt;\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_91_insufficient_backticks() {
    let test_html = Parser::render("``\nfoo\n``\n");
    let reference_html = "<p><code>foo</code></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_92_closing_fence_must_match_opening() {
    let test_html = Parser::render("```\naaa\n~~~\n```\n");
    let reference_html = "<pre><code>aaa\n~~~\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_93_closing_fence_with_tildes() {
    let test_html = Parser::render("~~~\naaa\n```\n~~~\n");
    let reference_html = "<pre><code>aaa\n```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_94_closing_fence_length_mismatch() {
    let test_html = Parser::render("````\naaa\n```\n``````\n");
    let reference_html = "<pre><code>aaa\n```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_95_closing_fence_with_extra_tildes() {
    let test_html = Parser::render("~~~~\naaa\n~~~\n~~~~\n");
    let reference_html = "<pre><code>aaa\n~~~\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_96_unclosed_code_block() {
    let test_html = Parser::render("```\n");
    let reference_html = "<pre><code></code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_97_nested_unclosed_code_block() {
    let test_html = Parser::render("`````\n\n```\naaa\n");
    let reference_html = "<pre><code>\n```\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_98_fenced_code_block_in_blockquote() {
    let test_html = Parser::render("> ```\n> aaa\n\nbbb\n");
    let reference_html = "<blockquote>\n<pre><code>aaa\n</code></pre>\n</blockquote>\n<p>bbb</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_99_code_block_with_empty_lines() {
    let test_html = Parser::render("```\n\n  \n```\n");
    let reference_html = "<pre><code>\n  \n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_100_empty_code_block() {
    let test_html = Parser::render("```\n```\n");
    let reference_html = "<pre><code></code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_101_fenced_code_block_with_indentation() {
    let test_html = Parser::render(" ```\n aaa\naaa\n```\n");
    let reference_html = "<pre><code>aaa\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_102_fenced_code_block_with_partial_indentation() {
    let test_html = Parser::render("  ```\naaa\n  aaa\naaa\n  ```\n");
    let reference_html = "<pre><code>aaa\naaa\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_103_indentation_and_closing_fence() {
    let test_html = Parser::render("   ```\n   aaa\n    aaa\n  aaa\n   ```\n");
    let reference_html = "<pre><code>aaa\n aaa\naaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_104_indented_code_instead_of_fenced_block() {
    let test_html = Parser::render("    ```\n    aaa\n    ```\n");
    let reference_html = "<pre><code>```\naaa\n```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_105_closing_fence_indented_less_than_opening() {
    let test_html = Parser::render("```\naaa\n  ```\n");
    let reference_html = "<pre><code>aaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_106_closing_fence_indented_by_three_spaces() {
    let test_html = Parser::render("   ```\naaa\n  ```\n");
    let reference_html = "<pre><code>aaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_107_closing_fence_indented_by_four_spaces() {
    let test_html = Parser::render("```\naaa\n    ```\n");
    let reference_html = "<pre><code>aaa\n    ```\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_108_internal_spaces_in_fence() {
    let test_html = Parser::render("``` ```\naaa\n");
    let reference_html = "<p><code> </code>\naaa</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_109_mismatched_fences() {
    let test_html = Parser::render("~~~~~~\naaa\n~~~ ~~~\n");
    let reference_html = "<pre><code>aaa\n~~~ ~~~\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_110_paragraphs_around_fenced_code_block() {
    let test_html = Parser::render("foo\n```\nbar\n```\nbaz\n");
    let reference_html =
        "<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_111_blocks_around_fenced_code_block() {
    let test_html = Parser::render("foo\n---\n~~~\nbar\n~~~\n# baz\n");
    let reference_html =
        "<h2>foo</h2>\n<pre><code>bar\n</code></pre>\n<h1>baz</h1>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_112_code_block_with_language_info_string() {
    let test_html =
        Parser::render("```ruby\ndef foo(x)\n  return 3\nend\n```\n");
    let reference_html =
        "<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_113_code_block_with_complex_info_string() {
    let test_html =
        Parser::render("~~~~    ruby startline=3 $%@#$\ndef foo(x)\n  return 3\nend\n~~~~~~~\n");
    let reference_html =
        "<pre><code class=\"language-ruby\">def foo(x)\n  return 3\nend\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_114_empty_info_string() {
    let test_html = Parser::render("````;\n````\n");
    let reference_html = "<pre><code class=\"language-;\"></code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_115_invalid_info_string_with_backticks() {
    let test_html = Parser::render("``` aa ```\nfoo\n");
    let reference_html = "<p><code>aa</code>\nfoo</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_116_info_string_with_backticks_and_tildes() {
    let test_html = Parser::render("~~~ aa ``` ~~~\nfoo\n~~~\n");
    let reference_html =
        "<pre><code class=\"language-aa\">foo\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_117_closing_fence_with_info_string() {
    let test_html = Parser::render("```\n``` aaa\n```\n");
    let reference_html = "<pre><code>``` aaa\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}
