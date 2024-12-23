use crate::error::{
	Error,
	Result,
};
use std::{
	env,
	path::PathBuf,
};

use headless_chrome::{
	protocol::cdp::Target::TargetID,
	types::PrintToPdfOptions,
	Browser,
};

/// Wrapper for interactions with the `headless_chrome` crate.
pub struct BrowserSession(Browser);

impl BrowserSession {
	/// Initializes a new browser session.
	pub fn initialize() -> Result<Self> {
		let browser = Browser::default().map_err(|_| {
			Error::BrowserError(
				"Failed to initialze headless_chrome session.".to_string(),
			)
		})?;

		Ok(BrowserSession(browser))
	}

	/// Opens an HTML file in a new browser tab and returns the tabs id.
	pub fn open_file(&self, path: &PathBuf) -> Result<TargetID> {
		let current_dir = env::current_dir()?;
		let html_file_path = current_dir.join(path);
		let file_url = format!("file://{}", html_file_path.to_string_lossy());

		let tab = self.0.new_tab().map_err(|_| {
			Error::BrowserError("Failed to open new tab.".to_string())
		})?;

		tab.navigate_to(&file_url).map_err(|_| {
			Error::BrowserError(
				"Failed to open html file.".to_string(),
			)
		})?;

		tab.wait_until_navigated().map_err(|_| {
			Error::BrowserError(
				"Failed to wait until navigation is done.".to_string(),
			)
		})?;

		Ok(tab.get_target_id().to_string())
	}

	/// Prints the content of a tab, identified by its id, to a PDF.
	pub fn print_to_pdf(
		&self,
		id: &TargetID,
		options: PrintToPdfOptions,
	) -> Result<Vec<u8>> {
		let tabs = self.0.get_tabs().lock().map_err(|_| {
			Error::BrowserError("Failed to lock tabs mutex".to_string())
		})?;

		let tab = tabs
			.iter()
			.find(|tab| tab.get_target_id() == id)
			.ok_or_else(|| {
				Error::BrowserError(format!(
					"No tab found with TargetID: {}",
					id
				))
			})?;

		let pdf_data = tab.print_to_pdf(Some(options)).map_err(|_| {
			Error::BrowserError("Failed to print to PDF:".to_string())
		})?;

		Ok(pdf_data)
	}
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_initialize() {
		let session = BrowserSession::initialize();
		assert!(session.is_ok());
	}

	#[test]
	fn test_open_file() {
		let session = BrowserSession::initialize()
			.expect("Failed to initialize browser session");
		let test_file_path = PathBuf::from("test.html");

		std::fs::write(&test_file_path, "<html><body>Test</body></html>")
			.expect("Failed to write test file");

		let result = session.open_file(&test_file_path);
		assert!(result.is_ok(), "Failed to open test HTML file");
	}

	#[test]
	fn test_print_to_pdf() {
		let session = BrowserSession::initialize()
			.expect("Failed to initialize browser session");
		let test_file_path = PathBuf::from("test.html");

		std::fs::write(&test_file_path, "<html><body>Test</body></html>")
			.expect("Failed to write test file");

		let target_id = session
			.open_file(&test_file_path)
			.expect("Failed to open test file");

		let pdf_options = PrintToPdfOptions::default();
		let pdf_data = session.print_to_pdf(&target_id, pdf_options);
		assert!(pdf_data.is_ok(), "Failed to print to PDF");
	}
}
