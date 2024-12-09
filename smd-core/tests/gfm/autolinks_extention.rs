use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_622_www_autolink() {
    let test_html = Parser::render("www.commonmark.org");
    let reference_html =
        "<p><a href=\"http://www.commonmark.org\">www.commonmark.org</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_623_www_autolink_with_path() {
    let test_html =
        Parser::render("Visit www.commonmark.org/help for more information.");
    let reference_html = "<p>Visit <a href=\"http://www.commonmark.org/help\">www.commonmark.org/help</a> for more information.</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_624_trailing_punctuation() {
    let test_html = Parser::render(
        "Visit www.commonmark.org.\n\nVisit www.commonmark.org/a.b.",
    );
    let reference_html = "<p>Visit <a href=\"http://www.commonmark.org\">www.commonmark.org</a>.</p>\n<p>Visit <a href=\"http://www.commonmark.org/a.b\">www.commonmark.org/a.b</a>.</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_625_parentheses_balancing() {
    let test_html = Parser::render("www.google.com/search?q=Markup+(business)\n\nwww.google.com/search?q=Markup+(business)))\n\n(www.google.com/search?q=Markup+(business))\n\n(www.google.com/search?q=Markup+(business)");
    let reference_html = "<p><a href=\"http://www.google.com/search?q=Markup+(business)\">www.google.com/search?q=Markup+(business)</a></p>\n<p><a href=\"http://www.google.com/search?q=Markup+(business)\">www.google.com/search?q=Markup+(business)</a>))</p>\n<p>(<a href=\"http://www.google.com/search?q=Markup+(business)\">www.google.com/search?q=Markup+(business)</a>)</p>\n<p>(<a href=\"http://www.google.com/search?q=Markup+(business)\">www.google.com/search?q=Markup+(business)</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_626_parentheses_interior() {
    let test_html = Parser::render("www.google.com/search?q=(business))+ok");
    let reference_html = "<p><a href=\"http://www.google.com/search?q=(business))+ok\">www.google.com/search?q=(business))+ok</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_627_entity_reference_in_autolink() {
    let test_html = Parser::render(
        "www.google.com/search?q=commonmark&hl=en\n\nwww.google.com/search?q=commonmark&hl;",
    );
    let reference_html = "<p><a href=\"http://www.google.com/search?q=commonmark&amp;hl=en\">www.google.com/search?q=commonmark&amp;hl=en</a></p>\n<p><a href=\"http://www.google.com/search?q=commonmark\">www.google.com/search?q=commonmark</a>&amp;hl;</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_628_autolink_with_lt() {
    let test_html = Parser::render("www.commonmark.org/he<lp");
    let reference_html =
        "<p><a href=\"http://www.commonmark.org/he\">www.commonmark.org/he</a>&lt;lp</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_629_http_https_autolink() {
    let test_html = Parser::render(
        "http://commonmark.org\n\n(Visit https://encrypted.google.com/search?q=Markup+(business))",
    );
    let reference_html = "<p><a href=\"http://commonmark.org\">http://commonmark.org</a></p>\n<p>(Visit <a href=\"https://encrypted.google.com/search?q=Markup+(business)\">https://encrypted.google.com/search?q=Markup+(business)</a>)</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_630_email_autolink() {
    let test_html = Parser::render("foo@bar.baz");
    let reference_html =
        "<p><a href=\"mailto:foo@bar.baz\">foo@bar.baz</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_631_email_plus_symbol() {
    let test_html = Parser::render(
        "hello@mail+xyz.example isn't valid, but hello+xyz@mail.example is.",
    );
    let reference_html = "<p>hello@mail+xyz.example isn't valid, but <a href=\"mailto:hello+xyz@mail.example\">hello+xyz@mail.example</a> is.</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_632_email_special_characters() {
    let test_html = Parser::render(
        "a.b-c_d@a.b\n\na.b-c_d@a.b.\n\na.b-c_d@a.b-\n\na.b-c_d@a.b_",
    );
    let reference_html = "<p><a href=\"mailto:a.b-c_d@a.b\">a.b-c_d@a.b</a></p>\n<p><a href=\"mailto:a.b-c_d@a.b\">a.b-c_d@a.b</a>.</p>\n<p>a.b-c_d@a.b-</p>\n<p>a.b-c_d@a.b_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_633_protocol_autolinks() {
    let test_html = Parser::render("mailto:foo@bar.baz\n\nmailto:a.b-c_d@a.b\n\nmailto:a.b-c_d@a.b.\n\nxmpp:foo@bar.baz\n\nxmpp:foo@bar.baz.");
    let reference_html = "<p><a href=\"mailto:foo@bar.baz\">mailto:foo@bar.baz</a></p>\n<p><a href=\"mailto:a.b-c_d@a.b\">mailto:a.b-c_d@a.b</a></p>\n<p><a href=\"mailto:a.b-c_d@a.b\">mailto:a.b-c_d@a.b</a>.</p>\n<p><a href=\"xmpp:foo@bar.baz\">xmpp:foo@bar.baz</a></p>\n<p><a href=\"xmpp:foo@bar.baz\">xmpp:foo@bar.baz</a>.</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_634_xmpp_autolink_with_resource() {
    let test_html = Parser::render(
        "xmpp:foo@bar.baz/txt\n\nxmpp:foo@bar.baz/txt@bin\n\nxmpp:foo@bar.baz/txt@bin.com",
    );
    let reference_html = "<p><a href=\"xmpp:foo@bar.baz/txt\">xmpp:foo@bar.baz/txt</a></p>\n<p><a href=\"xmpp:foo@bar.baz/txt@bin\">xmpp:foo@bar.baz/txt@bin</a></p>\n<p><a href=\"xmpp:foo@bar.baz/txt@bin.com\">xmpp:foo@bar.baz/txt@bin.com</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_635_xmpp_autolink_trailing_slash() {
    let test_html = Parser::render("xmpp:foo@bar.baz/txt/bin");
    let reference_html = "<p><a href=\"xmpp:foo@bar.baz/txt\">xmpp:foo@bar.baz/txt</a>/bin</p>\n";
    assert_eq!(test_html, reference_html);
}
