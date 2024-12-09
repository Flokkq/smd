use smd_core::gfm::parser::Parser;

#[test]
fn gfm_test_198_simple_table() {
    let test_html =
        Parser::render("| foo | bar |\n| --- | --- |\n| baz | bim |\n");
    let reference_html = "<table>\n<thead>\n<tr>\n<th>foo</th>\n<th>bar</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>baz</td>\n<td>bim</td>\n</tr>\n</tbody>\n</table>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_199_table_with_alignment() {
    let test_html =
        Parser::render("| abc | defghi |\n:-: | -----------:\nbar | baz\n");
    let reference_html = "<table>\n<thead>\n<tr>\n<th align=\"center\">abc</th>\n<th align=\"right\">defghi</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td align=\"center\">bar</td>\n<td align=\"right\">baz</td>\n</tr>\n</tbody>\n</table>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_200_table_with_escaped_pipes() {
    let test_html = Parser::render(
        "| f\\|oo  |\n| ------ |\n| b `\\|` az |\n| b **\\|** im |\n",
    );
    let reference_html = "<table>\n<thead>\n<tr>\n<th>f|oo</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>b <code>|</code> az</td>\n</tr>\n<tr>\n<td>b <strong>|</strong> im</td>\n</tr>\n</tbody>\n</table>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_201_table_ends_with_blockquote() {
    let test_html =
        Parser::render("| abc | def |\n| --- | --- |\n| bar | baz |\n> bar\n");
    let reference_html = "<table>\n<thead>\n<tr>\n<th>abc</th>\n<th>def</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>bar</td>\n<td>baz</td>\n</tr>\n</tbody>\n</table>\n<blockquote>\n<p>bar</p>\n</blockquote>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_202_table_with_extra_row_and_paragraph() {
    let test_html = Parser::render(
        "| abc | def |\n| --- | --- |\n| bar | baz |\nbar\n\nbar\n",
    );
    let reference_html = "<table>\n<thead>\n<tr>\n<th>abc</th>\n<th>def</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>bar</td>\n<td>baz</td>\n</tr>\n<tr>\n<td>bar</td>\n<td></td>\n</tr>\n</tbody>\n</table>\n<p>bar</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_203_table_not_recognized_with_mismatched_delimiter() {
    let test_html = Parser::render("| abc | def |\n| --- |\n| bar |\n");
    let reference_html = "<p>| abc | def |\n| --- |\n| bar |</p>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_204_table_with_varying_row_lengths() {
    let test_html = Parser::render(
        "| abc | def |\n| --- | --- |\n| bar |\n| bar | baz | boo |\n",
    );
    let reference_html = "<table>\n<thead>\n<tr>\n<th>abc</th>\n<th>def</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>bar</td>\n<td></td>\n</tr>\n<tr>\n<td>bar</td>\n<td>baz</td>\n</tr>\n</tbody>\n</table>\n";
    assert_eq!(test_html, reference_html);
}

#[test]
fn gfm_test_205_table_with_no_body_rows() {
    let test_html = Parser::render("| abc | def |\n| --- | --- |\n");
    let reference_html =
        "<table>\n<thead>\n<tr>\n<th>abc</th>\n<th>def</th>\n</tr>\n</thead>\n</table>\n";
    assert_eq!(test_html, reference_html);
}
