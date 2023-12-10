use reqwest::Url;
use std::{
    fs::File,
    io::{copy, BufReader},
    path::Path,
};
use zip::ZipArchive;

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

pub fn extract_and_delete_zip(zip_path: &str) -> std::io::Result<()> {
    // Open the zip file
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    let parent_dir = Path::new(zip_path).parent().unwrap_or(Path::new(""));

    // Extract each file in the zip
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = parent_dir.join(file.name());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = Path::new(&outpath).parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = File::create(&outpath)?;

            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    // Delete the original zip file
    std::fs::remove_file(zip_path)?;

    Ok(())
}
