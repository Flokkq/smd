use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_1_characters_and_lines_valid_document() {
    let test_html = Parser::render("→foo→baz→→bim");
    let reference_html = "<pre><code>foo→baz→→bim\n</code></pre>";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_2_characters_and_lines_indented_code_block() {
    let test_html = Parser::render("  →foo→baz→→bim");
    let reference_html = "<pre><code>foo→baz→→bim\n</code></pre>";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_3_unicode_in_code_block() {
    let test_html = Parser::render("    a→a\n    ὐ→a");
    let reference_html = "<pre><code>a→a\nὐ→a\n</code></pre>";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_4_list_with_tab_indentation() {
    let test_html = Parser::render("  - foo\n\n→bar");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_5_nested_list_with_tab_indentation() {
    let test_html = Parser::render("- foo\n\n→→bar");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<pre><code>  bar\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_6_block_quote_with_tab_indentation() {
    let test_html = Parser::render(">→→foo");
    let reference_html =
        "<blockquote>\n<pre><code>  foo\n</code></pre>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_7_list_item_with_tab_indentation() {
    let test_html = Parser::render("-→→foo");
    let reference_html =
        "<ul>\n<li>\n<pre><code>  foo\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_8_blank_line_in_code_block() {
    let test_html = Parser::render("    foo\n→bar");
    let reference_html = "<pre><code>foo\nbar\n</code></pre>";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_9_nested_list_with_indentation() {
    let test_html = Parser::render(" - foo\n   - bar\n→ - baz");
    let reference_html =
        "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_10_atx_heading_with_tab() {
    let test_html = Parser::render("#→Foo");
    let reference_html = "<h1>Foo</h1>";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_11_hr_with_tabs() {
    let test_html = Parser::render("*→*→*→");
    let reference_html = "<hr />";
    assert_eq!(test_html, reference_html);
}
