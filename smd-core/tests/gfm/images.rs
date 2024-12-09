use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_581_image_with_title() {
    let test_html = Parser::render("![foo](/url \"title\")");
    let reference_html =
        "<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_582_reference_image() {
    let test_html = Parser::render(
        "![foo *bar*]\n\n[foo *bar*]: train.jpg \"train & tracks\"",
    );
    let reference_html =
        "<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_583_nested_image_within_image() {
    let test_html = Parser::render("![foo ![bar](/url)](/url2)");
    let reference_html = "<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_584_link_within_image_description() {
    let test_html = Parser::render("![foo [bar](/url)](/url2)");
    let reference_html = "<p><img src=\"/url2\" alt=\"foo bar\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_585_collapsed_reference_image() {
    let test_html = Parser::render(
        "![foo *bar*][]\n\n[foo *bar*]: train.jpg \"train & tracks\"",
    );
    let reference_html =
        "<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_586_case_insensitive_reference_image() {
    let test_html = Parser::render(
        "![foo *bar*][foobar]\n\n[FOOBAR]: train.jpg \"train & tracks\"",
    );
    let reference_html =
        "<p><img src=\"train.jpg\" alt=\"foo bar\" title=\"train &amp; tracks\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_587_simple_inline_image() {
    let test_html = Parser::render("![foo](train.jpg)");
    let reference_html = "<p><img src=\"train.jpg\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_588_inline_image_with_spaces() {
    let test_html =
        Parser::render("My ![foo bar](/path/to/train.jpg  \"title\"   )");
    let reference_html =
        "<p>My <img src=\"/path/to/train.jpg\" alt=\"foo bar\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_589_image_with_angle_bracket_destination() {
    let test_html = Parser::render("![foo](<url>)");
    let reference_html = "<p><img src=\"url\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_590_image_with_empty_alt() {
    let test_html = Parser::render("![](/url)");
    let reference_html = "<p><img src=\"/url\" alt=\"\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_591_reference_style_image() {
    let test_html = Parser::render("![foo][bar]\n\n[bar]: /url");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_592_case_insensitive_reference_style_image() {
    let test_html = Parser::render("![foo][bar]\n\n[BAR]: /url");
    let reference_html = "<p><img src=\"/url\" alt=\"foo\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_593_collapsed_reference_image() {
    let test_html = Parser::render("![foo][]\n\n[foo]: /url \"title\"");
    let reference_html =
        "<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_594_collapsed_reference_with_formatting() {
    let test_html =
        Parser::render("![*foo* bar][]\n\n[*foo* bar]: /url \"title\"");
    let reference_html =
        "<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_595_case_insensitive_collapsed_reference() {
    let test_html = Parser::render("![Foo][]\n\n[foo]: /url \"title\"");
    let reference_html =
        "<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_596_whitespace_between_brackets_invalid() {
    let test_html = Parser::render("![foo] \n[]\n\n[foo]: /url \"title\"");
    let reference_html =
        "<p><img src=\"/url\" alt=\"foo\" title=\"title\" />\n[]</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_597_shortcut_reference_image() {
    let test_html = Parser::render("![foo]\n\n[foo]: /url \"title\"");
    let reference_html =
        "<p><img src=\"/url\" alt=\"foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_598_shortcut_reference_with_formatting() {
    let test_html =
        Parser::render("![*foo* bar]\n\n[*foo* bar]: /url \"title\"");
    let reference_html =
        "<p><img src=\"/url\" alt=\"foo bar\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_599_invalid_label_with_unescaped_brackets() {
    let test_html = Parser::render("![[foo]]\n\n[[foo]]: /url \"title\"");
    let reference_html =
        "<p>![[foo]]</p>\n<p>[[foo]]: /url &quot;title&quot;</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_600_case_insensitive_shortcut_reference() {
    let test_html = Parser::render("![Foo]\n\n[foo]: /url \"title\"");
    let reference_html =
        "<p><img src=\"/url\" alt=\"Foo\" title=\"title\" /></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_601_literal_with_escaped_bracket() {
    let test_html = Parser::render("!\\[foo]\n\n[foo]: /url \"title\"");
    let reference_html = "<p>![foo]</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_602_link_after_literal_exclamation() {
    let test_html = Parser::render("\\![foo]\n\n[foo]: /url \"title\"");
    let reference_html = "<p>!<a href=\"/url\" title=\"title\">foo</a></p>\n";
    assert_eq!(test_html, reference_html);
}
