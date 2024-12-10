use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_118_html_block_with_embedded_markdown() {
	let test_html = Parser::render(
		"<table><tr><td>\n<pre>\n**Hello**,\n\n_world_.\n</pre>\n</td></tr></\
		 table>\n",
	);
	let reference_html = "<table><tr><td>\n<pre>\n**Hello**,\n\n_world_.\n</\
	                      pre>\n</td></tr></table>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_119_basic_html_block_type_6() {
	let test_html = Parser::render(
		"<table>\n  <tr>\n    <td>\n           hi\n    </td>\n  \
		 </tr>\n</table>\n\nokay.\n",
	);
	let reference_html = "<table>\n  <tr>\n    <td>\n           hi\n    \
	                      </td>\n  </tr>\n</table>\n<p>okay.</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_120_html_block_with_unclosed_tag() {
	let test_html = Parser::render(" <div>\n  *hello*\n         <foo><a>\n");
	let reference_html = " <div>\n  *hello*\n         <foo><a>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_121_html_block_with_closing_tag() {
	let test_html = Parser::render("</div>\n*foo*\n");
	let reference_html = "</div>\n*foo*\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_122_html_block_with_markdown_between_tags() {
	let test_html =
		Parser::render("<DIV CLASS=\"foo\">\n\n*Markdown*\n\n</DIV>\n");
	let reference_html =
		"<DIV CLASS=\"foo\">\n<p><em>Markdown</em></p>\n</DIV>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_123_partial_opening_tag_html_block() {
	let test_html =
		Parser::render("<div id=\"foo\"\n  class=\"bar\">\n</div>\n");
	let reference_html = "<div id=\"foo\"\n  class=\"bar\">\n</div>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_124_partial_html_tag_with_multiline_attribute() {
	let test_html =
		Parser::render("<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n");
	let reference_html = "<div id=\"foo\" class=\"bar\n  baz\">\n</div>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_125_html_block_with_unclosed_tag_and_markdown() {
	let test_html = Parser::render("<div>\n*foo*\n\n*bar*\n");
	let reference_html = "<div>\n*foo*\n<p><em>bar</em></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_126_partial_html_tag_with_markdown() {
	let test_html = Parser::render("<div id=\"foo\"\n*hi*\n");
	let reference_html = "<div id=\"foo\"\n*hi*\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_127_partial_invalid_html_tag() {
	let test_html = Parser::render("<div class\nfoo\n");
	let reference_html = "<div class\nfoo\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_128_html_block_with_invalid_tag_and_markdown() {
	let test_html = Parser::render("<div *???-&&&-<---\n*foo*\n");
	let reference_html = "<div *???-&&&-<---\n*foo*\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_129_inline_html_block_with_markdown() {
	let test_html = Parser::render("<div><a href=\"bar\">*foo*</a></div>\n");
	let reference_html = "<div><a href=\"bar\">*foo*</a></div>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_130_html_block_with_embedded_text() {
	let test_html =
		Parser::render("<table><tr><td>\nfoo\n</td></tr></table>\n");
	let reference_html = "<table><tr><td>\nfoo\n</td></tr></table>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_131_html_block_with_markdown_inside() {
	let test_html = Parser::render("<div></div>\n``` c\nint x = 33;\n```\n");
	let reference_html = "<div></div>\n<pre><code class=\"language-c\">int x \
	                      = 33;\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_132_non_block_tag_html_with_markdown_inside() {
	let test_html = Parser::render("<a href=\"foo\">\n*bar*\n</a>\n");
	let reference_html = "<a href=\"foo\">\n*bar*\n</a>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_133_custom_html_block() {
	let test_html = Parser::render("<Warning>\n*bar*\n</Warning>\n");
	let reference_html = "<Warning>\n*bar*\n</Warning>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_134_inline_html_block_with_class() {
	let test_html = Parser::render("<i class=\"foo\">\n*bar*\n</i>\n");
	let reference_html = "<i class=\"foo\">\n*bar*\n</i>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_135_html_block_with_closing_tag_only() {
	let test_html = Parser::render("</ins>\n*bar*\n");
	let reference_html = "</ins>\n*bar*\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_136_html_block_with_del_tags() {
	let test_html = Parser::render("<del>\n*foo*\n</del>\n");
	let reference_html = "<del>\n*foo*\n</del>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_137_html_block_with_blank_line_inside() {
	let test_html = Parser::render("<del>\n\n*foo*\n\n</del>\n");
	let reference_html = "<del>\n<p><em>foo</em></p>\n</del>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_138_inline_html_inside_paragraph() {
	let test_html = Parser::render("<del>*foo*</del>\n");
	let reference_html = "<p><del><em>foo</em></del></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_139_html_block_pre_tag() {
	let test_html = Parser::render(
		"<pre language=\"haskell\"><code>\nimport Text.HTML.TagSoup\n\nmain \
		 :: IO ()\nmain = print $ parseTags tags\n</code></pre>\nokay\n",
	);
	let reference_html = "<pre language=\"haskell\"><code>\nimport \
	                      Text.HTML.TagSoup\n\nmain :: IO ()\nmain = print $ \
	                      parseTags tags\n</code></pre>\n<p>okay</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_140_html_block_script_tag() {
	let test_html = Parser::render(
		"<script type=\"text/javascript\">\n// JavaScript \
		 example\n\ndocument.getElementById(\"demo\").innerHTML = \"Hello \
		 JavaScript!\";\n</script>\nokay\n",
	);
	let reference_html = "<script type=\"text/javascript\">\n// JavaScript \
	                      example\n\ndocument.getElementById(\"demo\").\
	                      innerHTML = \"Hello \
	                      JavaScript!\";\n</script>\n<p>okay</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_141_html_block_style_tag() {
	let test_html = Parser::render(
		"<style\n  type=\"text/css\">\nh1 {color:red;}\n\np \
		 {color:blue;}\n</style>\nokay\n",
	);
	let reference_html = "<style\n  type=\"text/css\">\nh1 {color:red;}\n\np \
	                      {color:blue;}\n</style>\n<p>okay</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_142_html_block_no_matching_end_tag() {
	let test_html = Parser::render("<style\n  type=\"text/css\">\n\nfoo\n");
	let reference_html = "<style\n  type=\"text/css\">\n\nfoo\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_143_html_block_in_blockquote() {
	let test_html = Parser::render("> <div>\n> foo\n\nbar\n");
	let reference_html =
		"<blockquote>\n<div>\nfoo\n</blockquote>\n<p>bar</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_144_html_block_in_list() {
	let test_html = Parser::render("- <div>\n- foo\n");
	let reference_html = "<ul>\n<li>\n<div>\n</li>\n<li>foo</li>\n</ul>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_145_html_block_with_inline_content() {
	let test_html = Parser::render("<style>p{color:red;}</style>\n*foo*\n");
	let reference_html = "<style>p{color:red;}</style>\n<p><em>foo</em></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_146_html_comment_block() {
	let test_html = Parser::render("<!-- foo -->*bar*\n*baz*\n");
	let reference_html = "<!-- foo -->*bar*\n<p><em>baz</em></p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_147_html_block_with_inline_content_after() {
	let test_html = Parser::render("<script>\nfoo\n</script>1. *bar*\n");
	let reference_html = "<script>\nfoo\n</script>1. *bar*\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_148_html_block_with_comment_and_markdown() {
	let test_html = Parser::render("<!-- Foo\n\nbar\n   baz -->\nokay\n");
	let reference_html = "<!-- Foo\n\nbar\n   baz -->\n<p>okay</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_149_html_block_processing_instruction() {
	let test_html = Parser::render("<?php\n\n  echo '>';\n\n?>\nokay\n");
	let reference_html = "<?php\n\n  echo '>';\n\n?>\n<p>okay</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_150_html_block_doctype_declaration() {
	let test_html = Parser::render("<!DOCTYPE html>\n");
	let reference_html = "<!DOCTYPE html>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_151_html_block_cdata() {
	let test_html =
		Parser::render(
			"<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && a < 0) then \
			 {\n    return 1;\n\n  } else {\n\n    return 0;\n  \
			 }\n}\n]]>\nokay\n",
		);
	let reference_html = "<![CDATA[\nfunction matchwo(a,b)\n{\n  if (a < b && \
	                      a < 0) then {\n    return 1;\n\n  } else {\n\n    \
	                      return 0;\n  }\n}\n]]>\n<p>okay</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_152_indented_html_comment_block() {
	let test_html = Parser::render("  <!-- foo -->\n\n    <!-- foo -->\n");
	let reference_html =
		"  <!-- foo -->\n\n<pre><code>&lt;!-- foo --&gt;\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_153_indented_html_block() {
	let test_html = Parser::render("  <div>\n\n    <div>\n");
	let reference_html = "  <div>\n\n<pre><code>&lt;div&gt;\n</code></pre>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_154_html_block_interrupting_paragraph() {
	let test_html = Parser::render("Foo\n<div>\nbar\n</div>\n");
	let reference_html = "<p>Foo</p>\n<div>\nbar\n</div>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_155_html_block_needing_blank_line_after() {
	let test_html = Parser::render("<div>\nbar\n</div>\n*foo*\n");
	let reference_html = "<div>\nbar\n</div>\n*foo*\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_156_html_block_type_7_not_interrupting_paragraph() {
	let test_html = Parser::render("Foo\n<a href=\"bar\">\nbaz\n");
	let reference_html = "<p>Foo\n<a href=\"bar\">\nbaz</p>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_157_html_block_with_blank_line_inside() {
	let test_html = Parser::render("<div>\n\n*Emphasized* text.\n\n</div>\n");
	let reference_html = "<div>\n<p><em>Emphasized</em> text.</p>\n</div>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_158_html_block_without_blank_lines() {
	let test_html = Parser::render("<div>\n*Emphasized* text.\n</div>\n");
	let reference_html = "<div>\n*Emphasized* text.\n</div>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_159_html_block_with_nested_tags() {
	let test_html = Parser::render(
		"<table>\n\n<tr>\n\n<td>\nHi\n</td>\n\n</tr>\n\n</table>\n",
	);
	let reference_html = "<table>\n<tr>\n<td>\nHi\n</td>\n</tr>\n</table>\n";
	assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_160_html_block_with_nested_tags_and_indentation() {
	let test_html = Parser::render(
		"<table>\n\n  <tr>\n\n    <td>\n      Hi\n    </td>\n\n  \
		 </tr>\n\n</table>\n",
	);
	let reference_html = "<table>\n  <tr>\n<pre><code>&lt;td&gt;\n  \
	                      Hi\n&lt;/td&gt;\n</code></pre>\n  </tr>\n</table>\n";
	assert_eq!(test_html, reference_html);
}
