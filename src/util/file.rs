use std::{fs::File, io::copy, path::Path};

use reqwest::Url;

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).is_file()
}

#[allow(dead_code)] // TODO: Remove this, when feature to download and unzip is implemented
pub async fn download_file_to(url: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(Url::parse(url)?).await?;
    let dest_path = Path::new(dest);

    if let Some(parent) = dest_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let mut dest_file = File::create(dest_path)?;

    copy(&mut response.bytes().await?.as_ref(), &mut dest_file)?;

    Ok(())
}
