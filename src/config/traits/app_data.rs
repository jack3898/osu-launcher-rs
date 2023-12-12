use std::{
    path::{Path, PathBuf},
    process::Command,
};

use super::super::error::app_data_error::AppDataError;
use crate::{
    config::error::app_process_error::AppProcessError,
    util::file::{download_file_to, path_exists},
};
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
        if self.get_executable_path().is_err() {
            return false;
        }

        let executable_path = self.get_executable_path();

        match executable_path {
            Ok(path) => path_exists(path.to_str().unwrap()),
            Err(_) => false,
        }
    }

    fn get_executable_name(&self) -> Option<String> {
        None
    }

    fn get_public_download_url(&self) -> Option<String> {
        None
    }

    fn get_executable_path(&self) -> Result<PathBuf, AppDataError> {
        let path = self.get_path();
        let executable_name = self.get_executable_name();

        if path.is_none() {
            return Err(AppDataError::PathNotFound);
        }

        if executable_name.is_none() {
            return Err(AppDataError::ExecutableNameNotFound);
        }

        let executable_path = Path::new(&path.unwrap())
            .join(executable_name.unwrap())
            .to_string_lossy()
            .into_owned();

        Ok(PathBuf::from(executable_path))
    }

    fn can_download(&self) -> bool {
        self.get_enabled() && self.get_public_download_url().is_some() && !self.path_exists()
    }

    async fn download(&self) -> Result<PathBuf, AppDataError> {
        if !self.can_download() {
            return Err(AppDataError::DownloadDisabled);
        }

        let file_name = format!("{}.zip", uuid::Uuid::new_v4());

        let path_str = self.get_path().ok_or(AppDataError::PathNotFound)?;
        let download_location_path = Path::new(&path_str);

        let download_location = download_location_path
            .join(file_name)
            .to_string_lossy()
            .into_owned();

        let public_source_url = self
            .get_public_download_url()
            .ok_or(AppDataError::DownloadUrlNotFound)?;

        download_file_to(public_source_url.as_str(), &download_location)
            .await
            .or(Err(AppDataError::DownloadFailed))?;

        Ok(PathBuf::from(download_location))
    }

    fn try_spawn_process(
        &self,
    ) -> Result<JoinHandle<Result<std::process::ExitStatus, AppProcessError>>, AppProcessError>
    {
        if !self.get_enabled() || !self.executable_exists() {
            return Err(AppProcessError::AppNotFound);
        }

        if let Ok(executable_path) = self.get_executable_path() {
            let child_future = tokio::spawn(async move {
                let mut process = Command::new(executable_path).spawn().map_err(|err| {
                    AppProcessError::AppLaunchError(format!(
                        "Failed to launch process: {}",
                        err.to_string()
                    ))
                })?;

                process.wait().map_err(|_| AppProcessError::AppWaitError)
            });

            return Ok(child_future);
        }

        Err(AppProcessError::AppNotFound)
    }
}
