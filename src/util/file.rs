use std::{
    fs::File,
    io::{copy, BufReader},
    path::Path,
};

use reqwest::Url;
use zip::ZipArchive;

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).is_file()
}

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

pub fn extract_zip(zip_path: &str) -> std::io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    let parent_dir = Path::new(zip_path).parent().unwrap_or(Path::new(""));

    // Extract each file in the zip
    // Data in a zip file is stored as a single list of paths and files so we just loop through the zip's files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = parent_dir.join(file.name());

        // Check if the file is a directory
        if (&*file.name()).ends_with('/') {
            // If it is, create the directory and continue
            std::fs::create_dir_all(&outpath)?;
        } else {
            // If it isn't, create all of the parent directories and then create the file
            if let Some(p) = Path::new(&outpath).parent() {
                // If the parent doesn't exist create it
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = File::create(&outpath)?;

            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

pub fn delete_file(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path)
}
