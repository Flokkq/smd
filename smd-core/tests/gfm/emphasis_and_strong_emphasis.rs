use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_360_simple_emphasis() {
    let test_html = Parser::render("*foo bar*");
    let reference_html = "<p><em>foo bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_361_emphasis_whitespace_after_opening() {
    let test_html = Parser::render("a * foo bar*");
    let reference_html = "<p>a * foo bar*</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_362_emphasis_with_punctuation() {
    let test_html = Parser::render("a*\"foo\"*");
    let reference_html = "<p>a*&quot;foo&quot;*</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_363_unicode_whitespace_not_emphasis() {
    let test_html = Parser::render("* a *");
    let reference_html = "<p>* a *</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_364_intraword_emphasis_allowed() {
    let test_html = Parser::render("foo*bar*");
    let reference_html = "<p>foo<em>bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_365_intraword_numbers() {
    let test_html = Parser::render("5*6*78");
    let reference_html = "<p>5<em>6</em>78</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_366_simple_emphasis_with_underscore() {
    let test_html = Parser::render("_foo bar_");
    let reference_html = "<p><em>foo bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_367_emphasis_whitespace_after_opening_underscore() {
    let test_html = Parser::render("_ foo bar_");
    let reference_html = "<p>_ foo bar_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_368_emphasis_with_punctuation_underscore() {
    let test_html = Parser::render("a_\"foo\"_");
    let reference_html = "<p>a_&quot;foo&quot;_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_369_intraword_emphasis_disallowed_underscore() {
    let test_html = Parser::render("foo_bar_");
    let reference_html = "<p>foo_bar_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_370_numbers_underscore() {
    let test_html = Parser::render("5_6_78");
    let reference_html = "<p>5_6_78</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_371_unicode_text_underscore() {
    let test_html = Parser::render("пристаням_стремятся_");
    let reference_html = "<p>пристаням_стремятся_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_372_underscore_with_quotes() {
    let test_html = Parser::render("aa_\"bb\"_cc");
    let reference_html = "<p>aa_&quot;bb&quot;_cc</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_373_underscore_with_punctuation() {
    let test_html = Parser::render("foo-_(bar)_");
    let reference_html = "<p>foo-<em>(bar)</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_374_mismatched_delimiters() {
    let test_html = Parser::render("_foo*");
    let reference_html = "<p>_foo*</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_375_emphasis_closing_whitespace() {
    let test_html = Parser::render("*foo bar *");
    let reference_html = "<p>*foo bar *</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_376_newline_counts_as_whitespace() {
    let test_html = Parser::render("*foo bar\n*");
    let reference_html = "<p>*foo bar\n*</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_377_emphasis_with_punctuation_closing() {
    let test_html = Parser::render("*(*foo)");
    let reference_html = "<p>*(*foo)</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_378_nested_emphasis() {
    let test_html = Parser::render("*(*foo*)*");
    let reference_html = "<p><em>(<em>foo</em>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_379_intraword_emphasis() {
    let test_html = Parser::render("*foo*bar");
    let reference_html = "<p><em>foo</em>bar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_380_closing_underscore_whitespace() {
    let test_html = Parser::render("_foo bar _");
    let reference_html = "<p>_foo bar _</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_381_emphasis_with_punctuation_closing_underscore() {
    let test_html = Parser::render("_(_foo)");
    let reference_html = "<p>_(_foo)</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_382_nested_emphasis_with_underscore() {
    let test_html = Parser::render("_(_foo_)_");
    let reference_html = "<p><em>(<em>foo</em>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_383_intraword_emphasis_disallowed_with_underscore() {
    let test_html = Parser::render("_foo_bar");
    let reference_html = "<p>_foo_bar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_384_unicode_text_underscore_intraword_disallowed() {
    let test_html = Parser::render("_пристаням_стремятся");
    let reference_html = "<p>_пристаням_стремятся</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_385_emphasis_with_underscores() {
    let test_html = Parser::render("_foo_bar_baz_");
    let reference_html = "<p><em>foo_bar_baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_386_emphasis_followed_by_punctuation_underscore() {
    let test_html = Parser::render("_(bar)_.");
    let reference_html = "<p><em>(bar)</em>.</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_387_strong_emphasis() {
    let test_html = Parser::render("**foo bar**");
    let reference_html = "<p><strong>foo bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_388_strong_emphasis_whitespace_after_opening() {
    let test_html = Parser::render("** foo bar**");
    let reference_html = "<p>** foo bar**</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_389_strong_emphasis_with_punctuation() {
    let test_html = Parser::render("a**\"foo\"**");
    let reference_html = "<p>a**&quot;foo&quot;**</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_390_intraword_strong_emphasis() {
    let test_html = Parser::render("foo**bar**");
    let reference_html = "<p>foo<strong>bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_391_strong_emphasis_underscores() {
    let test_html = Parser::render("__foo bar__");
    let reference_html = "<p><strong>foo bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_392_strong_emphasis_whitespace_after_opening_underscores() {
    let test_html = Parser::render("__ foo bar__");
    let reference_html = "<p>__ foo bar__</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_393_strong_emphasis_with_newline() {
    let test_html = Parser::render("__\nfoo bar__");
    let reference_html = "<p>__\nfoo bar__</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_394_strong_emphasis_with_punctuation_underscores() {
    let test_html = Parser::render("a__\"foo\"__");
    let reference_html = "<p>a__&quot;foo&quot;__</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_395_intraword_strong_emphasis_disallowed_with_underscores() {
    let test_html = Parser::render("foo__bar__");
    let reference_html = "<p>foo__bar__</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_396_numbers_with_strong_emphasis_underscores() {
    let test_html = Parser::render("5__6__78");
    let reference_html = "<p>5__6__78</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_397_unicode_text_with_strong_emphasis() {
    let test_html = Parser::render("пристаням__стремятся__");
    let reference_html = "<p>пристаням__стремятся__</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_398_nested_strong_emphasis() {
    let test_html = Parser::render("__foo, __bar__, baz__");
    let reference_html =
        "<p><strong>foo, <strong>bar</strong>, baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_399_strong_emphasis_with_punctuation() {
    let test_html = Parser::render("foo-__(bar)__");
    let reference_html = "<p>foo-<strong>(bar)</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_400_closing_strong_emphasis_whitespace() {
    let test_html = Parser::render("**foo bar **");
    let reference_html = "<p>**foo bar **</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_401_strong_emphasis_with_punctuation_in_closing() {
    let test_html = Parser::render("**(**foo)");
    let reference_html = "<p>**(**foo)</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_402_nested_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("*(**foo**)*)");
    let reference_html = "<p><em>(<strong>foo</strong>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_403_combined_emphasis_with_citations() {
    let test_html = Parser::render(
        "**Gomphocarpus (*Gomphocarpus physocarpus*, syn.\n*Asclepias physocarpa*)**",
    );
    let reference_html = "<p><strong>Gomphocarpus (<em>Gomphocarpus physocarpus</em>, syn.\n<em>Asclepias physocarpa</em>)</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_404_emphasis_with_quotes() {
    let test_html = Parser::render("**foo \"*bar*\" foo**");
    let reference_html =
        "<p><strong>foo &quot;<em>bar</em>&quot; foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_405_intraword_strong_emphasis() {
    let test_html = Parser::render("**foo**bar");
    let reference_html = "<p><strong>foo</strong>bar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_406_strong_emphasis_with_closing_whitespace() {
    let test_html = Parser::render("__foo bar __");
    let reference_html = "<p>__foo bar __</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_407_strong_emphasis_with_punctuation_and_underscore() {
    let test_html = Parser::render("__(__foo)");
    let reference_html = "<p>__(__foo)</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_408_nested_emphasis_with_strong_emphasis() {
    let test_html = Parser::render("_(__foo__)_");
    let reference_html = "<p><em>(<strong>foo</strong>)</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_409_intraword_strong_emphasis_disallowed_with_underscores() {
    let test_html = Parser::render("__foo__bar");
    let reference_html = "<p>__foo__bar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_410_unicode_with_intraword_strong_emphasis() {
    let test_html = Parser::render("__пристаням__стремятся");
    let reference_html = "<p>__пристаням__стремятся</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_411_nested_strong_emphasis_underscore() {
    let test_html = Parser::render("__foo__bar__baz__");
    let reference_html = "<p><strong>foo__bar__baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_412_strong_emphasis_followed_by_punctuation() {
    let test_html = Parser::render("__(bar)__.");
    let reference_html = "<p><strong>(bar)</strong>.</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_413_emphasis_with_links() {
    let test_html = Parser::render("*foo [bar](/url)*");
    let reference_html = "<p><em>foo <a href=\"/url\">bar</a></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_414_emphasis_across_lines() {
    let test_html = Parser::render("*foo\nbar*");
    let reference_html = "<p><em>foo\nbar</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_415_nested_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("_foo __bar__ baz_");
    let reference_html = "<p><em>foo <strong>bar</strong> baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_416_nested_emphasis() {
    let test_html = Parser::render("_foo _bar_ baz_");
    let reference_html = "<p><em>foo <em>bar</em> baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_417_double_nested_emphasis() {
    let test_html = Parser::render("__foo_ bar_");
    let reference_html = "<p><em><em>foo</em> bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_418_emphasis_with_extra_closing_delimiter() {
    let test_html = Parser::render("*foo *bar**");
    let reference_html = "<p><em>foo <em>bar</em></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_419_nested_emphasis_and_strong_emphasis_mixed() {
    let test_html = Parser::render("*foo **bar** baz*");
    let reference_html = "<p><em>foo <strong>bar</strong> baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_420_intraword_emphasis_and_strong_emphasis_mixed() {
    let test_html = Parser::render("*foo**bar**baz*");
    let reference_html = "<p><em>foo<strong>bar</strong>baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_421_unmatched_emphasis() {
    let test_html = Parser::render("*foo**bar*");
    let reference_html = "<p><em>foo**bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_422_nested_strong_emphasis_in_emphasis() {
    let test_html = Parser::render("***foo** bar*");
    let reference_html = "<p><em><strong>foo</strong> bar</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_423_nested_strong_emphasis_in_emphasis_variant() {
    let test_html = Parser::render("*foo **bar***");
    let reference_html = "<p><em>foo <strong>bar</strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_424_nested_strong_emphasis_in_emphasis_variant_2() {
    let test_html = Parser::render("*foo**bar***");
    let reference_html = "<p><em>foo<strong>bar</strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_425_nested_emphasis_with_balanced_delimiters() {
    let test_html = Parser::render("foo***bar***baz");
    let reference_html = "<p>foo<em><strong>bar</strong></em>baz</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_426_nested_emphasis_with_unbalanced_delimiters() {
    let test_html = Parser::render("foo******bar*********baz");
    let reference_html = "<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_427_multilevel_nested_emphasis() {
    let test_html = Parser::render("*foo **bar *baz* bim** bop*");
    let reference_html =
        "<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_428_nested_emphasis_with_links() {
    let test_html = Parser::render("*foo [*bar*](/url)*");
    let reference_html =
        "<p><em>foo <a href=\"/url\"><em>bar</em></a></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_429_no_empty_emphasis() {
    let test_html = Parser::render("** is not an empty emphasis");
    let reference_html = "<p>** is not an empty emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_430_no_empty_strong_emphasis() {
    let test_html = Parser::render("**** is not an empty strong emphasis");
    let reference_html = "<p>**** is not an empty strong emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_431_strong_emphasis_with_links() {
    let test_html = Parser::render("**foo [bar](/url)**");
    let reference_html =
        "<p><strong>foo <a href=\"/url\">bar</a></strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_432_multiline_strong_emphasis() {
    let test_html = Parser::render("**foo\nbar**");
    let reference_html = "<p><strong>foo\nbar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_433_nested_emphasis_in_strong_emphasis() {
    let test_html = Parser::render("__foo _bar_ baz__");
    let reference_html = "<p><strong>foo <em>bar</em> baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_434_double_nested_strong_emphasis() {
    let test_html = Parser::render("__foo __bar__ baz__");
    let reference_html =
        "<p><strong>foo <strong>bar</strong> baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_435_triple_nested_strong_emphasis() {
    let test_html = Parser::render("____foo__ bar__");
    let reference_html = "<p><strong><strong>foo</strong> bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_436_nested_strong_emphasis_with_unmatched_delimiters() {
    let test_html = Parser::render("**foo **bar****");
    let reference_html = "<p><strong>foo <strong>bar</strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_437_nested_emphasis_within_strong_emphasis() {
    let test_html = Parser::render("**foo *bar* baz**");
    let reference_html = "<p><strong>foo <em>bar</em> baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_438_intraword_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("**foo*bar*baz**");
    let reference_html = "<p><strong>foo<em>bar</em>baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_439_emphasis_within_strong_emphasis_variant() {
    let test_html = Parser::render("***foo* bar**");
    let reference_html = "<p><strong><em>foo</em> bar</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_440_nested_emphasis_in_strong_emphasis_variant() {
    let test_html = Parser::render("**foo *bar***");
    let reference_html = "<p><strong>foo <em>bar</em></strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_441_multilevel_nested_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("**foo *bar **baz**\nbim* bop**");
    let reference_html = "<p><strong>foo <em>bar <strong>baz</strong>\nbim</em> bop</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_442_nested_emphasis_with_links_in_strong_emphasis() {
    let test_html = Parser::render("**foo [*bar*](/url)**");
    let reference_html =
        "<p><strong>foo <a href=\"/url\"><em>bar</em></a></strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_443_no_empty_emphasis_variant() {
    let test_html = Parser::render("__ is not an empty emphasis");
    let reference_html = "<p>__ is not an empty emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_444_no_empty_strong_emphasis_variant() {
    let test_html = Parser::render("____ is not an empty strong emphasis");
    let reference_html = "<p>____ is not an empty strong emphasis</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_445_literal_asterisks() {
    let test_html = Parser::render("foo ***");
    let reference_html = "<p>foo ***</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_446_emphasis_with_literal_asterisk() {
    let test_html = Parser::render("foo *\\**");
    let reference_html = "<p>foo <em>*</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_447_emphasis_with_literal_underscore() {
    let test_html = Parser::render("foo *_*");
    let reference_html = "<p>foo <em>_</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_448_literal_asterisks_with_unmatched_delimiters() {
    let test_html = Parser::render("foo *****");
    let reference_html = "<p>foo *****</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_449_strong_emphasis_with_literal_asterisk() {
    let test_html = Parser::render("foo **\\***");
    let reference_html = "<p>foo <strong>*</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_450_strong_emphasis_with_literal_underscore() {
    let test_html = Parser::render("foo **_**");
    let reference_html = "<p>foo <strong>_</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_451_excess_literal_asterisks_outside_emphasis() {
    let test_html = Parser::render("**foo*");
    let reference_html = "<p>*<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_452_excess_literal_asterisks_outside_emphasis_reverse() {
    let test_html = Parser::render("*foo**");
    let reference_html = "<p><em>foo</em>*</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_453_nested_strong_emphasis_with_excess_literal_asterisks() {
    let test_html = Parser::render("***foo**");
    let reference_html = "<p>*<strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_454_excess_literal_asterisks_in_emphasis() {
    let test_html = Parser::render("****foo*");
    let reference_html = "<p>***<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_455_strong_emphasis_with_excess_literal_asterisks() {
    let test_html = Parser::render("**foo***");
    let reference_html = "<p><strong>foo</strong>*</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_456_emphasis_with_excess_literal_asterisks() {
    let test_html = Parser::render("*foo****");
    let reference_html = "<p><em>foo</em>***</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_457_excess_literal_underscores() {
    let test_html = Parser::render("foo ___");
    let reference_html = "<p>foo ___</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_458_emphasis_with_literal_underscore() {
    let test_html = Parser::render("foo _\\__");
    let reference_html = "<p>foo <em>_</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_459_emphasis_with_literal_asterisk() {
    let test_html = Parser::render("foo _*_");
    let reference_html = "<p>foo <em>*</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_460_excess_literal_underscores_with_unmatched_delimiters() {
    let test_html = Parser::render("foo _____");
    let reference_html = "<p>foo _____</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_461_strong_emphasis_with_literal_underscore() {
    let test_html = Parser::render("foo __\\___");
    let reference_html = "<p>foo <strong>_</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_462_strong_emphasis_with_literal_asterisk() {
    let test_html = Parser::render("foo __*__");
    let reference_html = "<p>foo <strong>*</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_463_excess_literal_underscores_outside_emphasis() {
    let test_html = Parser::render("__foo_");
    let reference_html = "<p>_<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_464_excess_literal_underscores_outside_emphasis_reverse() {
    let test_html = Parser::render("_foo__");
    let reference_html = "<p><em>foo</em>_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_465_nested_strong_emphasis_with_excess_literal_underscores() {
    let test_html = Parser::render("___foo__");
    let reference_html = "<p>_<strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_466_excess_literal_underscores_in_emphasis() {
    let test_html = Parser::render("____foo_");
    let reference_html = "<p>___<em>foo</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_467_strong_emphasis_with_excess_literal_underscores() {
    let test_html = Parser::render("__foo___");
    let reference_html = "<p><strong>foo</strong>_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_468_emphasis_with_excess_literal_underscores() {
    let test_html = Parser::render("_foo____");
    let reference_html = "<p><em>foo</em>___</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_469_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("**foo**");
    let reference_html = "<p><strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_470_nested_emphasis_with_different_delimiters() {
    let test_html = Parser::render("*_foo_*");
    let reference_html = "<p><em><em>foo</em></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_471_strong_emphasis_and_nested_emphasis() {
    let test_html = Parser::render("__foo__");
    let reference_html = "<p><strong>foo</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_472_nested_emphasis_with_different_delimiters_variant() {
    let test_html = Parser::render("_*foo*_");
    let reference_html = "<p><em><em>foo</em></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_473_double_nested_strong_emphasis() {
    let test_html = Parser::render("****foo****");
    let reference_html = "<p><strong><strong>foo</strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_474_double_nested_strong_emphasis_variant() {
    let test_html = Parser::render("____foo____");
    let reference_html = "<p><strong><strong>foo</strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_475_triple_nested_strong_emphasis() {
    let test_html = Parser::render("******foo******");
    let reference_html =
        "<p><strong><strong><strong>foo</strong></strong></strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_476_emphasis_with_strong_emphasis() {
    let test_html = Parser::render("***foo***");
    let reference_html = "<p><em><strong>foo</strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_477_triple_nested_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("_____foo_____");
    let reference_html =
        "<p><em><strong><strong>foo</strong></strong></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_478_mismatched_emphasis_delimiters() {
    let test_html = Parser::render("*foo _bar* baz_");
    let reference_html = "<p><em>foo _bar</em> baz_</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_479_mixed_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("*foo __bar *baz bim__ bam*");
    let reference_html =
        "<p><em>foo <strong>bar *baz bim</strong> bam</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_480_unmatched_nested_emphasis_and_strong_emphasis() {
    let test_html = Parser::render("**foo **bar baz**");
    let reference_html = "<p>**foo <strong>bar baz</strong></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_481_unmatched_nested_emphasis() {
    let test_html = Parser::render("*foo *bar baz*");
    let reference_html = "<p>*foo <em>bar baz</em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_482_emphasis_with_unmatched_link_delimiters() {
    let test_html = Parser::render("*[bar*](/url)");
    let reference_html = "<p>*<a href=\"/url\">bar*</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_483_emphasis_with_unmatched_link_delimiters_and_underscore() {
    let test_html = Parser::render("_foo [bar_](/url)");
    let reference_html = "<p>_foo <a href=\"/url\">bar_</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_484_emphasis_with_html_image_tag() {
    let test_html = Parser::render("*<img src=\"foo\" title=\"*\"/>");
    let reference_html = "<p>*<img src=\"foo\" title=\"*\"/></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_485_strong_emphasis_with_html_link_tag() {
    let test_html = Parser::render("**<a href=\"**\">");
    let reference_html = "<p>**<a href=\"**\"></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_486_strong_emphasis_with_html_link_tag_variant() {
    let test_html = Parser::render("__<a href=\"__\">");
    let reference_html = "<p>__<a href=\"__\"></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_487_emphasis_with_inline_code() {
    let test_html = Parser::render("*a `*`*");
    let reference_html = "<p><em>a <code>*</code></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_488_emphasis_with_inline_code_and_underscore() {
    let test_html = Parser::render("_a `_`_");
    let reference_html = "<p><em>a <code>_</code></em></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_489_strong_emphasis_with_autolink() {
    let test_html = Parser::render("**a<http://foo.bar/?q=**>");
    let reference_html =
        "<p>**a<a href=\"http://foo.bar/?q=**\">http://foo.bar/?q=**</a></p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_490_strong_emphasis_with_autolink_and_underscore() {
    let test_html = Parser::render("__a<http://foo.bar/?q=__>");
    let reference_html =
        "<p>__a<a href=\"http://foo.bar/?q=__\">http://foo.bar/?q=__</a></p>\n";
    assert_eq!(test_html, reference_html);
}
