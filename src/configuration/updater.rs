use crate::api::UpdateInfo;
use std::io::{Read, Write};
use std::path::PathBuf;
use tokio::task;
use tokio::{fs, io::AsyncWriteExt};
use zip::ZipArchive;

pub async fn update(
    configuration_dir: &PathBuf,
    update_info: UpdateInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let permission_message = if cfg!(target_os = "windows") {
        "Update might need Administrator permissions to replace the executable. Please ensure this program is run as an administrator."
    } else {
        "Update might require sudo permissions to replace the executable. Please ensure this program is run with 'sudo' if necessary."
    };
    println!("{}", permission_message);

    let response = reqwest::get(&update_info.download_url)
        .await?
        .bytes()
        .await?;
    let temp_file_path = configuration_dir.join("extracted").join("update.zip");
    let mut temp_file = fs::File::create(&temp_file_path).await?;
    temp_file.write_all(&response).await?;

    let mut archive = ZipArchive::new(std::fs::File::open(&temp_file_path)?)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        let outpath = configuration_dir.join("extracted").join(&file_name);
        if file_name.ends_with('/') {
            fs::create_dir_all(&outpath).await?;
            continue;
        }

        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)?;

        task::spawn_blocking(move || {
            let mut outfile = std::fs::File::create(&outpath)?;
            outfile.write_all(&file_contents)?;
            Result::<(), std::io::Error>::Ok(())
        })
        .await??;
    }

    let executable_name = "smd";
    let target_path = if cfg!(target_os = "windows") {
        PathBuf::from("C:\\Windows\\System32")
    } else {
        PathBuf::from("/usr/local/bin/")
    }
    .join(executable_name);

    let new_executable_path =
        configuration_dir.join("extracted").join(executable_name);
    fs::rename(new_executable_path, target_path).await?;

    Ok(())
}
