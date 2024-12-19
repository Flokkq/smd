use std::{
	env,
	fs,
};

use headless_chrome::types::PrintToPdfOptions;

use crate::{
	browser::BrowserSession,
	error::Result,
	fs::write_to_file,
};

/// Trait for converting html to other file formats.
pub trait Convert {
	fn from_html(html: &str) -> Result<Vec<u8>>;
}

pub fn convert_html<T>(html: &str) -> Result<Vec<u8>>
where
	T: Convert,
{
	T::from_html(html)
}

pub struct PDFConverter;

impl Convert for PDFConverter {
	fn from_html(html: &str) -> Result<Vec<u8>> {
		let browser = BrowserSession::initialize()?;

		let current_dir = env::current_dir()?;
		let tmpfile = current_dir.join("tmp.html");

		write_to_file(&tmpfile, html)?;
		let tab_id = browser.open_file(&tmpfile)?;
		fs::remove_file(tmpfile)?;

		let options = PrintToPdfOptions {
			margin_top: Some(0.0),
			margin_bottom: Some(0.0),
			margin_left: Some(0.0),
			margin_right: Some(0.0),
			print_background: Some(true),
			..Default::default()
		};
		let converted = browser.print_to_pdf(&tab_id, options)?;

		Ok(converted)
	}
}
