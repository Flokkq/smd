use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_12_block_structure_precedence() {
	let test_html = Parser::render("- `one\n- two`");
	let reference_html = "<ul>\n<li>`one</li>\n<li>two`</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}
