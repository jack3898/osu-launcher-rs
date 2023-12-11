use std::{path::Path, process::Command};

use crate::util::file::{download_file_to, path_exists};
use async_trait::async_trait;
use tokio::task::JoinHandle;

#[async_trait]
pub trait Application {
    fn get_enabled(&self) -> bool {
        false
    }

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

    fn can_download(&self) -> bool {
        self.get_enabled() && self.get_public_download_url().is_some() && !self.path_exists()
    }

    async fn download(&self) -> Result<String, String> {
        if !self.can_download() {
            return Err(String::from(
                "Cannot download! Check the config is correct.",
            ));
        }

        let file_name = format!("{}.zip", uuid::Uuid::new_v4());

        let download_location = Path::new(&self.get_path().unwrap())
            .join(file_name)
            .to_string_lossy()
            .into_owned();

        let download = download_file_to(
            self.get_public_download_url().unwrap().as_str(),
            &download_location,
        )
        .await;

        if download.is_err() {
            return Err(format!("Error downloading: {}", download.err().unwrap()));
        }

        Ok(download_location)
    }

    fn try_spawn_process(&self) -> Option<JoinHandle<()>> {
        if !self.get_enabled() || !self.executable_exists() {
            return None;
        }

        if let Some(executable_path) = self.get_executable_path() {
            let child_future = tokio::spawn(async move {
                let mut process = Command::new(executable_path)
                    .spawn()
                    .expect("Failed to launch!");

                process.wait().expect("Failed to wait for process!");
            });

            return Some(child_future);
        }

        None
    }
}
