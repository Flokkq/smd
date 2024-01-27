// TODO: allow multiple arguments for --input:
//      smd --input readme.md test/testCase.md ../mylittlepony.md --output img --specific png

// TODO: implement margin for page end.
// TODO: capture entire screen for img, or multiple screen sized images
// TODO: implement correct handling of files including
//      slashes
//      dots
//      etc.

use std::path::PathBuf;
use crate::configuration::Settings;

fn parse_to_md(settings: Settings, path: PathBuf) {

}