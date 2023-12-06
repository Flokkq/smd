use headless_chrome::types;
use std::process::exit;

use crate::{config, utils, fio};

pub fn parse_md(filename: &str, output_type: &str, specific_type: Option<&str>) {
    match config::validate_inputs(filename, output_type, specific_type) {
        Ok(_) => (),
        Err(msg) => println!("{}", msg)
    }

    let md_content = match fio::read_file(filename.into()) {
        Ok(content) => content,
        Err(_) => return
    };

    let html_filename = format!("{}.html", filename.split(".").next().unwrap());
    utils::parse_md_to_html(&md_content, &html_filename);

    match output_type {
        "html" => (),
        "pdf" => {
            let (_browser, tab) = utils::initialize_browser(&html_filename);
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
        utils::remove_html_file(html_filename.into());    
        },
        "img" => {
            use headless_chrome::protocol::cdp::Page;
            match specific_type {
                Some("png") => {
                    let (_browser, tab) = utils::initialize_browser(&html_filename);
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
                    let (_browser, tab) = utils::initialize_browser(&html_filename);
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
                    let (_browser, tab) = utils::initialize_browser(&html_filename);
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
        utils::remove_html_file(html_filename.into());    
        },
        _ => ()
    }
}
