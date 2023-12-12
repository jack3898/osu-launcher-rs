#[derive(Debug)]
pub enum AppDataError {
    PathNotFound,
    ExecutableNameNotFound,
    DownloadDisabled,
    DownloadFailed,
    DownloadUrlNotFound,
}

impl std::fmt::Display for AppDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppDataError::PathNotFound => write!(f, "Path not found."),
            AppDataError::ExecutableNameNotFound => write!(f, "Executable name not found."),
            AppDataError::DownloadDisabled => write!(f, "Download for this app is disabled."),
            AppDataError::DownloadFailed => write!(f, "Download failed."),
            AppDataError::DownloadUrlNotFound => write!(f, "Download URL not found."),
        }
    }
}

impl std::error::Error for AppDataError {}
