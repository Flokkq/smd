use std::error::Error;
use std::{env, fs};
use std::fs::{File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, exit};
use std::result::Result;
use headless_chrome::{Browser, types, Tab};
use std::sync::Arc;



pub enum MdFlavour {
    Light,
    Dark,
    Auto
}

impl MdFlavour {
    pub fn to_string(&self) -> String {
        match self {
            MdFlavour::Light => "light".to_string(),
            MdFlavour::Dark => "dark".to_string(),
            MdFlavour::Auto => "auto".to_string()
        }
    }
}
pub fn set_md_flavour(flavour: MdFlavour) {
    let filepath = get_path_to_config_file();

    match fs::write(filepath, flavour.to_string()) {
        Ok(_) => println!("INFO: set md flavour to {}", flavour.to_string()),
        Err(err) => {
            eprintln!("ERROR: could not set md flavour to {}: {}", flavour.to_string(), err);
            exit(1);
        }
    }
}

pub fn parse_md(filename: &str, output_type: &str, specific_type: Option<&str>) {
    match validate_inputs(filename, output_type, specific_type) {
        Ok(_) => (),
        Err(msg) => println!("{}", msg)
    }

    let md_content = match read_file(filename) {
        Ok(content) => content,
        Err(_) => return
    };

    let html_filename = format!("{}.html", filename.split(".").next().unwrap());
    parse_md_to_html(&md_content, &html_filename);
    
    match output_type {
        "html" => (),
        "pdf" => { 
            let (_browser, tab) = initialize_browser(&html_filename);
            let options = types::PrintToPdfOptions {
                print_background: Some(true),
                margin_top: Some(0.0),
                margin_bottom: Some(0.0),
                margin_left: Some(0.0),
                margin_right: Some(0.0),
                ..Default::default()
            };

            match tab.print_to_pdf(Some(options)) {
                Ok(pdf_data) => {
                    std::fs::write(format!("{}.pdf", filename.split(".").next().unwrap()), pdf_data).expect("Failed writing Pdf");
                },
                Err(err) => {
                    eprintln!("ERROR: could not generate pdf data: {}", err);
                    exit(1);
                }
            }
        },
        "img" => {
            use headless_chrome::protocol::cdp::Page;
            match specific_type {
                Some("png") => {
                    let (_browser, tab) = initialize_browser(&html_filename);
                    match tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true) {
                        Ok(png_data) => {
                            std::fs::write(format!("{}.png", filename.split(".").next().unwrap()), png_data).expect("Failed drawing Image");
                        },
                        Err(err) => {
                            eprintln!("ERROR: could not generate image data: {}", err);
                        }
                    }
                },
                Some("jpeg") => {
                    let (_browser, tab) = initialize_browser(&html_filename);
                    match tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true) {
                        Ok(jpeg_data) => {
                            std::fs::write(format!("{}.jpeg", filename.split(".").next().unwrap()), jpeg_data).expect("Failed drawing Image");
                        },
                        Err(err) => {
                            eprintln!("ERROR: could not generate image data: {}", err);
                        }
                    }
                },
                Some("webp") => {
                    let (_browser, tab) = initialize_browser(&html_filename);
                    match tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Webp, None, None, true) {
                        Ok(webp_data) => {
                            std::fs::write(format!("{}.webp", filename.split(".").next().unwrap()), webp_data).expect("Failed drawing Image");
                        },
                        Err(err) => {
                            eprintln!("ERROR: could not generate image data: {}", err);
                        }
                    }
                },
                Some(&_) => (),
                None => ()
            }
        },
        _ => ()
    }
}

fn initialize_browser(html_filename: &str) -> (Browser, Arc<Tab>) { 
    let browser = Browser::default().expect("Failed to start browser");
    
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let html_file_path = current_dir.join(html_filename);

    if !html_file_path.exists() {
        eprintln!("ERROR: HTML file not found");
        exit(1);
     }
    let file_url = format!("file://{}", html_file_path.to_string_lossy());

     let tab = browser.new_tab().unwrap();
     tab.navigate_to(&file_url).expect("Failed to navigate to URL");
     tab.wait_until_navigated().expect("Failed to wait for navigation");
     return (browser, tab);
}

pub fn check_requirements() {
    println!("INFO Checking requirements...");

    let npm_check = Command::new("npm")
        .arg("ls")
        .arg("-g")
        .output();

    match npm_check {
        Ok(output) => {
            let response = String::from_utf8_lossy(&output.stdout);
            if !response.contains(" github-markdown-css") {
                println!("INFO Installing github-markdown-css...");
                match Command::new("npm")
                    .arg("install")
                    .arg("-g")
                    .arg("github-markdown-css")
                    .output() {
                    Ok(_) => println!("INFO All requirements are fulfilled! Continuing..."),
                    Err(err) => {
                        eprintln!("ERROR installing github-markdown-css: {}", err);
                        exit(1);
                    }
                }
            }
            println!("INFO All requirements are fulfilled! Continuing...");
        }
        Err(err) => {
            eprintln!("ERROR checking requirements: {}", err)
        }
    }

    let file_path = get_path_to_config_file();
    if !file_path.exists() {
        fs::create_dir(&file_path).expect("ERROR creating directory");
        File::create(file_path.join("md_flavour.txt")).expect("ERROR creating file");
    }
}

fn validate_inputs(filename: &str, output_type: &str, specific_type: Option<&str>) -> Result<(), Box<dyn Error>> {
    if filename.is_empty() {
        return Err("ERROR: no input file specified".into());
    }

    let current_dir = env::current_dir().expect("ERROR resolving current directory");
    if !current_dir.join(filename).exists() {
        let error_msg = format!("ERROR: file {} does not exist", filename);
        return Err(error_msg.into())
    }

    match output_type {
        "html" | "pdf" | "img" => (),
        _ => return Err("ERROR: invalid output type".into())
    }

    match specific_type {
        Some(str) => {
            match str {
                "jpeg" | "png" | "webp" => (),
                &_ => return Err("ERROR: invalid specific type".into())
            }
        },
        None => ()
    }

    return Ok(());
}

fn read_file(filename: &str) -> Result<String, ()> {
    println!("INFO Reading file: {}", filename);
    return fs::read_to_string(filename).map_err(|err| {
        eprintln!("ERROR: could not open file {}: {err}", filename, );
    })
}

fn write_file(filename: &str, content: &str) {
    println!("INFO Writing file: {}", filename);
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: could not create file {}: {}", filename, err);
            exit(1);
        }
    };

    file.write_all(content.as_bytes()).map_err(|err| {
        eprintln!("ERROR: could not write to file {}: {}", filename, err)
    }).expect("TODO: panic message");
}

fn parse_md_to_html(md_content: &str, filename: &str) {
    println!("INFO Parsing {} to html", filename);

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
            let html_content = String::from_utf8_lossy(&output.stdout);
            let css_path;

            match Command::new("npm").arg("root").arg("-g").output() {
                Ok(output) => {
                    let npm_root = String::from_utf8_lossy(&output.stdout);
                    let npm_root = npm_root.trim();
                    let css_path_str = format!("{}/github-markdown-css/github-markdown-dark.css", npm_root);
                    
                    // only iterates to ~
                    let mut absolute_css_path = String::new();
                    for _ in css_path_str.split('/') {
                        absolute_css_path.push_str("../");
                    }
                    #[cfg(unix)]
                        absolute_css_path.push_str("../../");

                    css_path = absolute_css_path.clone() + css_path_str.trim_start_matches("/");
                    println!("{}", css_path);
                },
                _ => {
                    eprintln!("ERROR: unable to retrieve npm root");
                    exit(1);
                }
            }

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
             <body class=\"markdown-body\">\n\
                     {}\n\
             </body>\n\
             ", css_path, html_content);

            write_file(&filename, &html_structure);
        },
        Err(err) => {
            eprintln!("ERROR: could not parse md to html: {}", err);
            exit(1);
        }
    };
}

fn get_path_to_config_file() -> PathBuf {
    let home_dir = dirs::home_dir().expect("ERROR getting home directory");
    #[cfg(unix)]
        let dir_path = home_dir.join("Library/Smd");
    #[cfg(windows)]
        let dir_path = home_dir.join("AppData/Roaming/Smd");
    let file_path = dir_path.join("md_flavour.txt");

    return file_path
}