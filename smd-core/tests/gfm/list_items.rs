use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_231_blocks_outside_list() {
    let test_html =
        Parser::render("A paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.\n");
    let reference_html = "<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_232_ordered_list_item() {
    let test_html = Parser::render(
        "1.  A paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.\n",
    );
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_233_unindented_paragraph_after_list() {
    let test_html = Parser::render("- one\n\n two\n");
    let reference_html = "<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_234_indented_paragraph_in_list() {
    let test_html = Parser::render("- one\n\n  two\n");
    let reference_html = "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_235_indented_code_block_after_list_item() {
    let test_html = Parser::render(" -    one\n\n     two\n");
    let reference_html =
        "<ul>\n<li>one</li>\n</ul>\n<pre><code> two\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_236_properly_indented_paragraph_in_list() {
    let test_html = Parser::render(" -    one\n\n      two\n");
    let reference_html = "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_237_indentation_in_nested_blocks() {
    let test_html = Parser::render("   > > 1.  one\n>>\n>>     two\n");
    let reference_html = "<blockquote>\n<blockquote>\n<ol>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ol>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_238_insufficiently_indented_paragraph_in_nested_blocks() {
    let test_html = Parser::render(">>- one\n>>\n  >  > two\n");
    let reference_html = "<blockquote>\n<blockquote>\n<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n</blockquote>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_239_invalid_list_items() {
    let test_html = Parser::render("-one\n\n2.two\n");
    let reference_html = "<p>-one</p>\n<p>2.two</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_240_list_item_with_multiple_blank_lines() {
    let test_html = Parser::render("- foo\n\n\n  bar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_241_complex_list_item() {
    let test_html = Parser::render(
        "1.  foo\n\n    ```\n    bar\n    ```\n\n    baz\n\n    > bam\n",
    );
    let reference_html = "<ol>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n<p>baz</p>\n<blockquote>\n<p>bam</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_242_preserve_blank_lines_in_code_block() {
    let test_html = Parser::render("- Foo\n\n      bar\n\n\n      baz\n");
    let reference_html =
        "<ul>\n<li>\n<p>Foo</p>\n<pre><code>bar\n\n\nbaz\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_243_large_start_number_in_ordered_list() {
    let test_html = Parser::render("123456789. ok\n");
    let reference_html = "<ol start=\"123456789\">\n<li>ok</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_244_too_large_start_number_in_ordered_list() {
    let test_html = Parser::render("1234567890. not ok\n");
    let reference_html = "<p>1234567890. not ok</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_245_ordered_list_with_zero_as_start() {
    let test_html = Parser::render("0. ok\n");
    let reference_html = "<ol start=\"0\">\n<li>ok</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_246_ordered_list_with_leading_zeros() {
    let test_html = Parser::render("003. ok\n");
    let reference_html = "<ol start=\"3\">\n<li>ok</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_247_negative_number_in_ordered_list() {
    let test_html = Parser::render("-1. not ok\n");
    let reference_html = "<p>-1. not ok</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_248_indented_code_in_list_item() {
    let test_html = Parser::render("- foo\n\n      bar\n");
    let reference_html =
        "<ul>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_249_deeply_indented_code_in_list_item() {
    let test_html = Parser::render("  10.  foo\n\n           bar\n");
    let reference_html =
        "<ol start=\"10\">\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_250_indented_code_outside_list() {
    let test_html =
        Parser::render("    indented code\n\nparagraph\n\n    more code\n");
    let reference_html = "<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_251_indented_code_as_list_item() {
    let test_html = Parser::render(
        "1.     indented code\n\n   paragraph\n\n       more code\n",
    );
    let reference_html = "<ol>\n<li>\n<pre><code>indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_252_extra_indent_in_code_block() {
    let test_html = Parser::render(
        "1.      indented code\n\n   paragraph\n\n       more code\n",
    );
    let reference_html = "<ol>\n<li>\n<pre><code> indented code\n</code></pre>\n<p>paragraph</p>\n<pre><code>more code\n</code></pre>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_253_no_list_items_with_partial_indent() {
    let test_html = Parser::render("   foo\n\nbar\n");
    let reference_html = "<p>foo</p>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_254_invalid_list_due_to_insufficient_indentation() {
    let test_html = Parser::render("-    foo\n\n  bar\n");
    let reference_html = "<ul>\n<li>foo</li>\n</ul>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_255_valid_list_with_correct_indentation() {
    let test_html = Parser::render("-  foo\n\n   bar\n");
    let reference_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_256_list_with_blank_lines() {
    let test_html =
        Parser::render("-\n  foo\n-\n  ```\n  bar\n  ```\n-\n      baz\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li>\n<pre><code>bar\n</code></pre>\n</li>\n<li>\n<pre><code>baz\n</code></pre>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_257_indentation_does_not_change_blank_line_behavior() {
    let test_html = Parser::render("-   \n  foo\n");
    let reference_html = "<ul>\n<li>foo</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_258_list_with_multiple_blank_lines_stopping_item() {
    let test_html = Parser::render("-\n\n  foo\n");
    let reference_html = "<ul>\n<li></li>\n</ul>\n<p>foo</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_259_empty_bullet_list_items() {
    let test_html = Parser::render("- foo\n-\n- bar\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_260_list_with_mixed_spaces() {
    let test_html = Parser::render("- foo\n-   \n- bar\n");
    let reference_html = "<ul>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_261_empty_ordered_list_items() {
    let test_html = Parser::render("1. foo\n2.\n3. bar\n");
    let reference_html = "<ol>\n<li>foo</li>\n<li></li>\n<li>bar</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_262_empty_bullet_list() {
    let test_html = Parser::render("*\n");
    let reference_html = "<ul>\n<li></li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_263_empty_list_does_not_interrupt_paragraph() {
    let test_html = Parser::render("foo\n*\n\nfoo\n1.\n");
    let reference_html = "<p>foo\n*</p>\n<p>foo\n1.</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_264_one_space_indented_list_item() {
    let test_html = Parser::render(" 1.  A paragraph\n     with two lines.\n\n         indented code\n\n     > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_265_two_spaces_indented_list_item() {
    let test_html = Parser::render("  1.  A paragraph\n      with two lines.\n\n          indented code\n\n      > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_266_three_spaces_indented_list_item() {
    let test_html = Parser::render("   1.  A paragraph\n       with two lines.\n\n           indented code\n\n       > A block quote.\n");
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_267_four_spaces_is_code_block() {
    let test_html = Parser::render("    1.  A paragraph\n        with two lines.\n\n            indented code\n\n        > A block quote.\n");
    let reference_html = "<pre><code>1.  A paragraph\n    with two lines.\n\n        indented code\n\n    &gt; A block quote.\n</code></pre>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_268_lazy_continuation_in_list() {
    let test_html = Parser::render(
        "  1.  A paragraph\nwith two lines.\n\n          indented code\n\n      > A block quote.\n",
    );
    let reference_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_269_partial_deletion_of_indentation() {
    let test_html = Parser::render("  1.  A paragraph\n    with two lines.\n");
    let reference_html = "<ol>\n<li>A paragraph\nwith two lines.</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_270_nested_structure_lazy_continuation() {
    let test_html = Parser::render("> 1. > Blockquote\ncontinued here.\n");
    let reference_html = "<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_271_nested_structure_full_continuation() {
    let test_html = Parser::render("> 1. > Blockquote\n> continued here.\n");
    let reference_html = "<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_272_properly_nested_sublists() {
    let test_html = Parser::render("- foo\n  - bar\n    - baz\n      - boo\n");
    let reference_html = "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz\n<ul>\n<li>boo</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_273_insufficiently_nested_sublists() {
    let test_html = Parser::render("- foo\n - bar\n  - baz\n   - boo\n");
    let reference_html =
        "<ul>\n<li>foo</li>\n<li>bar</li>\n<li>baz</li>\n<li>boo</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_274_sublist_under_wider_marker() {
    let test_html = Parser::render("10) foo\n    - bar\n");
    let reference_html =
        "<ol start=\"10\">\n<li>foo\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_275_insufficient_indentation_with_wider_marker() {
    let test_html = Parser::render("10) foo\n   - bar\n");
    let reference_html =
        "<ol start=\"10\">\n<li>foo</li>\n</ol>\n<ul>\n<li>bar</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_276_list_as_first_block_in_item() {
    let test_html = Parser::render("- - foo\n");
    let reference_html =
        "<ul>\n<li>\n<ul>\n<li>foo</li>\n</ul>\n</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_277_nested_list_with_ordered_list() {
    let test_html = Parser::render("1. - 2. foo\n");
    let reference_html = "<ol>\n<li>\n<ul>\n<li>\n<ol start=\"2\">\n<li>foo</li>\n</ol>\n</li>\n</ul>\n</li>\n</ol>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_278_list_item_with_heading_and_rules() {
    let test_html = Parser::render("- # Foo\n- Bar\n  ---\n  baz\n");
    let reference_html = "<ul>\n<li>\n<h1>Foo</h1>\n</li>\n<li>\n<h2>Bar</h2>\nbaz</li>\n</ul>\n";
    assert_eq!(test_html, reference_html);
}
