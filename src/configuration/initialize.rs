use std::fs::{copy, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use reqwest::{Client, Url};
use crate::configuration::{get_flavours_from_config_folder};
use crate::io::write_file;

pub async fn initialize() {
    let config_folder = dirs::config_dir().unwrap().join("smd");
    log::info!("Creating configuration folder: {:?}", config_folder);

    std::fs::create_dir(config_folder.clone()).unwrap();
    std::fs::create_dir(config_folder.join("flavours")).unwrap();

    let default_flavours = vec![
        ("https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.5.0/github-markdown.css", config_folder.join("flavours").join("github-markdown.css")),
        ("https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.5.0/github-markdown-dark.css", config_folder.join("flavours").join("github-markdown-dark.css")),
        ("https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.5.0/github-markdown-light.css", config_folder.join("flavours").join("github-markdown-light.css")),
    ];

    log::info!("Downloading default flavours...");
    for (url, save_path) in default_flavours {
        download_file(Url::from_str(url).unwrap(), save_path).await;
    }

    let flavours = get_flavours_from_config_folder(config_folder.join("flavours")
        .clone())
        .unwrap()
        .join("\n");

    write_file(config_folder.join("current-flavour.css"), "auto".to_string());
    write_file(config_folder.join("flavours.txt"), flavours);
    log::info!("Initialization complete! Run smd --help for more information.");
    std::process::exit(0);
}

async fn download_file(url: Url, save_path: PathBuf) {
    let client = Client::new();

    let response = client.get(url).send().await.unwrap();
    if !response.status().is_success() {
        reset_initialization();
    }

    let mut file = File::create(save_path.clone()).unwrap();
    let mut content = vec![];
    content.extend(response.bytes().await.unwrap());
    file.write_all(&content).unwrap();
}

fn reset_initialization() {
    log::error!("Failed loading standard flavours. Please rerun the initialization!");
    std::process::exit(1);
}