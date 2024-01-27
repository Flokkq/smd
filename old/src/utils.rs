use crate::{fio, mdflavour};
use headless_chrome::{Browser, Tab};
use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::sync::Arc;

pub const VERSION: &str = "0.1.0";
pub const RED_CODE: &str = "\x1b[31m";
pub const BLUE_CODE: &str = "\x1b[34m";
pub const MAGENTA_CODE: &str = "\x1b[35m";
pub const NORMAL_CODE: &str = "\x1b[0m";

pub fn initialize_browser(html_filename: &str) -> (Browser, Arc<Tab>) {
    let browser = Browser::default().expect("Failed to start browser");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let html_file_path = current_dir.join(html_filename);

    if !html_file_path.exists() {
        eprintln!("ERROR: HTML file not found");
        exit(1);
    }
    let file_url = format!("file://{}", html_file_path.to_string_lossy());

    let tab = browser.new_tab().unwrap();
    tab.navigate_to(&file_url)
        .expect("Failed to navigate to URL");
    tab.wait_until_navigated()
        .expect("Failed to wait for navigation");
    return (browser, tab);
}

pub fn parse_md_to_html(md_content: &str, filename: &str) {
    // println!("INFO Parsing {} to html", filename);

    let command = Command::new("gh")
        .arg("api")
        .arg("--method")
        .arg("POST")
        .arg("-H")
        .arg("Accept: application/vnd.github+json")
        .arg("-H")
        .arg("X-GitHub-Api-Version: 2022-11-28")
        .arg("/markdown")
        .arg("-f")
        .arg(format!("text={}", md_content))
        .output();

    match command {
        Ok(output) => {
            let mut html_content = String::from_utf8_lossy(&output.stdout);
            let css_path;

            match Command::new("npm").arg("root").arg("-g").output() {
                Ok(output) => {
                    let npm_root = String::from_utf8_lossy(&output.stdout);
                    let npm_root = npm_root.trim();
                    let md_flavour = mdflavour::get_md_flavour();
                    let css_path_str = format!(
                        "{}/github-markdown-css/github-markdown{}.css",
                        npm_root, md_flavour
                    );

                    // only iterates to ~
                    let mut absolute_css_path = String::new();
                    for _ in css_path_str.split('/') {
                        absolute_css_path.push_str("../");
                    }
                    #[cfg(unix)]
                    absolute_css_path.push_str("../../");

                    css_path = absolute_css_path.clone() + css_path_str.trim_start_matches("/");
                }
                Err(err) => {
                    eprintln!("ERROR: unable to retrieve npm root: {}", err);
                    exit(1);
                }
            }

            let fixed_html_content = fix_html_inner_document_links(&html_content);
            let html_structure = format!(
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
                 <link rel=\"stylesheet\" href={:?}>\n\
                 <style>\n\
                     .markdown-body {{\n\
                         box-sizing: border-box;\n\
                         margin: 0 auto;\n\
                         padding: 45px;\n\
                     }}\n\
                     \n\
                     @media (max-width: 767px) {{\n\
                         .markdown-body {{\n\
                             padding: 15px;\n\
                         }}\n\
                     }}\n\
                 </style>\n\
             <body>\n\
                    <main class=\"markdown-body\">\n\
                        {}\n\
                    </main>\n\
             </body>\n\
             ",
                css_path, fixed_html_content
            );

            let path = PathBuf::from(filename);
            fio::write_file(path, &html_structure);
        }
        Err(err) => {
            eprintln!("ERROR: could not parse md to html: {}", err);
            exit(1);
        }
    };
}

fn fix_html_inner_document_links(mut html_content: &str) -> String {
    let href_prefix = "href=\"#user-content-";
    let mut fixed_html_content = String::new();

    for line in html_content.split_terminator("\n") {
        if line.contains("href=\"#") {
            let mut line = line.to_string();
            line = line.replace("href=\"#", href_prefix);

            fixed_html_content.push_str(&line);
        } else {
            fixed_html_content.push_str(&line);
        }
    }
    fixed_html_content
}

pub fn remove_html_file(filepath: PathBuf) {
    match std::fs::remove_file(filepath) {
        Ok(_) => (),
        Err(err) => eprintln!("ERROR removing file: {}", err),
    }
}
