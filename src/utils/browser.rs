use headless_chrome::{Browser, Tab};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

pub struct WebBrowserSession {
    pub browser: Browser,
    pub tab: Arc<Tab>,
}

impl WebBrowserSession {
    pub async fn initialize(html_file: PathBuf) -> Result<Self, String> {
        let browser = Browser::default().map_err(|_| "Failed to start browser")?;

        let current_dir = env::current_dir().map_err(|_| "Failed to get current directory")?;
        let html_file_path = current_dir.join(html_file);

        if !html_file_path.exists() {
            return Err("ERROR: HTML file not found".to_string());
        }
        let file_url = format!("file://{}", html_file_path.to_string_lossy());

        let tab = browser.new_tab().map_err(|_| "Failed to open a new tab")?;
        tab.navigate_to(&file_url)
            .map_err(|_| "Failed to navigate to URL")?;
        tab.wait_until_navigated()
            .map_err(|_| "Failed to wait until navigation is done")?;

        Ok(WebBrowserSession { browser, tab })
    }
}
