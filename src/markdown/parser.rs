use headless_chrome::types::PrintToPdfOptions;
use reqwest::Client;

use crate::configuration::Settings;
use crate::io::{read_file, write_file};
use crate::utils::WebBrowserSession;
use std::error::Error;
use std::{path::PathBuf, process::exit};

// TODO: allow multiple arguments for --input:
//      smd --input readme.md test/testCase.md ../mylittlepony.md --output img --specific png

// TODO: implement margin for page end
// TODO: capture entire screen for img, or multiple screen sized images
// TODO: implement correct handling of files including
//      slashes
//      dots
//      etc.
// TODO: add a yaml file, to customize font, color, etc. and build the css file from that.
//     - this would make flavours/github-markdown.css the 'standard'
//     - meaning that a css and yaml file will be in smd/flavours

pub async fn parse(
    settings: Settings,
    path: PathBuf,
    output_type: &str,
    specific_type: Option<&str>,
) {
    if !path.exists() {
        log::error!("File {} does not exist", path.to_str().unwrap());
        exit(1);
    }

    if !(path.extension().unwrap().to_str() == Some("md")) {
        log::error!(
            "{} is not of markdown filetype. Please provide a correct file",
            path.to_str().unwrap()
        );
        exit(1);
    }

    let html_content = match read_file(path.clone()) {
        Ok(content) => content,
        Err(err) => {
            log::error!(
                "Could not read contents from {} : {:?}",
                path.to_str().unwrap(),
                err
            );
            exit(1);
        }
    };

    let html_filename = format!("{}.html", path.file_stem().unwrap().to_str().unwrap());
    parse_md_to_html(settings, html_filename.clone().into(), html_content).await;

    match output_type {
        "html" => (),
        "pdf" => {
            let session = WebBrowserSession::initialize(html_filename.into())
                .await
                .unwrap();
            let options = PrintToPdfOptions {
                margin_top: Some(0.0),
                margin_bottom: Some(0.0),
                margin_left: Some(0.0),
                margin_right: Some(0.0),
                print_background: Some(true),
                ..Default::default()
            };

            if let Ok(pdf) = session.tab.print_to_pdf(Some(options)) {
                std::fs::write(
                    format!("{}.pdf", path.file_stem().unwrap().to_str().unwrap()),
                    pdf,
                )
                .expect("Could not write pdf file");
            } else {
                log::error!("Could not convert html to pdf");
                exit(1);
            }
        }
        "img" => {}
        _ => (),
    }
}

async fn parse_md_to_html(settings: Settings, path: PathBuf, content: String) {
    let html_content = match render_markdown(&content).await {
        Ok(content) => content,
        Err(err) => {
            log::error!("{}", format!("Could not parse markdown to html: {:?}", err));
            exit(1);
        }
    };

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
        settings.config_folder.join("current-flavour.css"),
        html_content
    );
    write_file(path, html_structure);
}

async fn render_markdown(content: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .post("http:127.0.0.1:8080/render")
        .body(content.to_string())
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response.text().await.unwrap())
    } else {
        let error_message = format!(
            "Failed to fetch HTML content: {} - {}",
            response.status(),
            response.text().await?
        );
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        )))
    }
}
