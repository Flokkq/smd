use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_279_task_list_items_basic() {
	let test_html = Parser::render("- [ ] foo\n- [x] bar\n");
	let reference_html = "<ul>\n<li><input disabled=\"\" type=\"checkbox\"> \
	                      foo</li>\n<li><input checked=\"\" disabled=\"\" \
	                      type=\"checkbox\"> bar</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_280_nested_task_list_items() {
	let test_html =
		Parser::render("- [x] foo\n  - [ ] bar\n  - [x] baz\n- [ ] bim\n");
	let reference_html =
		"<ul>\n<li><input checked=\"\" disabled=\"\" type=\"checkbox\"> \
		 foo\n<ul>\n<li><input disabled=\"\" type=\"checkbox\"> \
		 bar</li>\n<li><input checked=\"\" disabled=\"\" type=\"checkbox\"> \
		 baz</li>\n</ul>\n</li>\n<li><input disabled=\"\" type=\"checkbox\"> \
		 bim</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}
