use std::{path::PathBuf, process::exit};

use headless_chrome::{
    protocol::cdp::Page::CaptureScreenshotFormatOption,
    types::PrintToPdfOptions,
};

use crate::{
    api::APICommunicator,
    browser::WebBrowserSession,
    configuration::configuration::{APIConfiguration, Settings},
    error::invalid_argument_message,
    file_access::{FileAccess, WriteOperation},
};

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

    let markdown_content = match FileAccess::read_file(path.clone()) {
        Ok(content) => content,
        Err(_) => {
            exit(1);
        }
    };

    let html_filename =
        format!("{}.html", path.file_stem().unwrap().to_str().unwrap());

    parse_md_to_html(
        &settings.application.configuration_dir,
        settings.api,
        PathBuf::from(&html_filename),
        markdown_content,
    )
    .await;

    match output_type {
        "html" => (),
        "pdf" => {
            let session =
                WebBrowserSession::initialize(&PathBuf::from(html_filename))
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
                    format!(
                        "{}.pdf",
                        path.file_stem().unwrap().to_str().unwrap()
                    ),
                    pdf,
                )
                .unwrap()
            } else {
                log::error!("Could not convert markdown to pdf");
                exit(1);
            }
        }
        "img" => {
            if let Some(specific_type) = specific_type {
                convert_md_to_image(
                    &PathBuf::from(html_filename),
                    specific_type.to_string(),
                )
                .await;
            }
        }
        _ => invalid_argument_message(),
    }
}

async fn parse_md_to_html(
    configuration_dir: &PathBuf,
    api_configuration: APIConfiguration,
    path: PathBuf,
    content: String,
) {
    let html_content = match render_markdown(api_configuration, content).await {
        Ok(content) => content,
        Err(err) => {
            log::error!(
                "{}",
                format!("Could not parse markdown to html: {:?}", err)
            );
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
        configuration_dir.join("current-flavour.css"),
        html_content
    );
    FileAccess::write_file(path, html_structure, WriteOperation::Write)
        .unwrap();
}

async fn render_markdown(
    api_configuration: APIConfiguration,
    content: String,
) -> Result<String, anyhow::Error> {
    let api_communitacor = APICommunicator::build(api_configuration);
    api_communitacor.post_markdown(&content).await
}

async fn convert_md_to_image(path: &PathBuf, image_type: String) {
    let web_browser_session =
        WebBrowserSession::initialize(&PathBuf::from(path))
            .await
            .expect("Failed creating img");

    let capture_screenshot = |options: CaptureScreenshotFormatOption| {
        let img_data = web_browser_session
            .tab
            .capture_screenshot(options, None, None, true);

        match img_data {
            Ok(png_data) => png_data,
            Err(err) => {
                eprintln!("ERROR: could not generate image data: {}", err);
                exit(1);
            }
        }
    };

    let img_data = match image_type.as_str() {
        "png" => capture_screenshot(CaptureScreenshotFormatOption::Png),
        "jpg" | "jpeg" => {
            capture_screenshot(CaptureScreenshotFormatOption::Jpeg)
        }
        "webp" => capture_screenshot(CaptureScreenshotFormatOption::Webp),
        _ => {
            log::warn!("Not a valid option");
            exit(1);
        }
    };

    std::fs::write(
        format!(
            "{}.{}",
            path.file_stem().unwrap().to_str().unwrap(),
            image_type
        ),
        img_data,
    )
    .expect("Failed saving image");
}
