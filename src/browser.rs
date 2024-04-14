use headless_chrome::{Browser, Tab};
use std::convert::Infallible;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

pub struct WebBrowserSession {
    pub browser: Browser,
    pub tab: Arc<Tab>,
}

impl WebBrowserSession {
    pub async fn initialize(html_file: &PathBuf) -> Result<Self, Infallible> {
        let browser = Browser::default().unwrap();

        let current_dir = env::current_dir().unwrap();
        let html_file_path = current_dir.join(html_file);
        let file_url = format!("file://{}", html_file_path.to_string_lossy());

        let tab = browser
            .new_tab()
            .map_err(|_| "Failed to open a new tab")
            .unwrap();

        tab.navigate_to(&file_url)
            .map_err(|_| "Failed to navigate to URL")
            .unwrap();

        tab.wait_until_navigated()
            .map_err(|_| "Failed to wait until navigation is done")
            .unwrap();

        Ok(WebBrowserSession { browser, tab })
    }
}
