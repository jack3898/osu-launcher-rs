use std::path::Path;

use crate::util::file::{download_file_to, extract_and_delete_zip, path_exists};
use async_trait::async_trait;

#[async_trait]
pub trait AppData {
    fn get_path(&self) -> Option<String> {
        None
    }

    fn path_exists(&self) -> bool {
        if self.get_path().is_none() {
            return false;
        }

        let path = self.get_path().unwrap();

        path_exists(&path)
    }

    fn executable_exists(&self) -> bool {
        if self.get_executable_path().is_none() {
            return false;
        }

        let executable_path = self.get_executable_path().unwrap();

        path_exists(&executable_path)
    }

    fn get_executable_name(&self) -> Option<String> {
        None
    }

    fn get_public_download_url(&self) -> Option<String> {
        None
    }

    fn get_executable_path(&self) -> Option<String> {
        let path = self.get_path();
        let executable_name = self.get_executable_name();

        if path.is_none() || executable_name.is_none() {
            return None;
        }

        let executable_path = Path::new(&path.unwrap())
            .join(executable_name.unwrap())
            .to_string_lossy()
            .into_owned();

        Some(executable_path)
    }

    async fn download_application(&self) -> Result<(), String> {
        let file_name = format!("{}.zip", uuid::Uuid::new_v4());

        if self.get_path().is_none() {
            return Err(String::from("Local path is not set"));
        }

        if self.get_public_download_url().is_none() {
            return Err(String::from("Public download url is not set"));
        }

        let download_destination = Path::new(&self.get_path().unwrap())
            .join(file_name)
            .to_string_lossy()
            .into_owned();

        let download = download_file_to(
            self.get_public_download_url().unwrap().as_str(),
            &download_destination,
        )
        .await;

        if download.is_err() {
            return Err(format!("Error downloading: {}", download.err().unwrap()));
        }

        extract_and_delete_zip(&download_destination).unwrap();

        Ok(())
    }
}
