use std::fmt;

#[derive(Debug)]
pub enum AppProcessError {
    AppWaitError,
    AppLaunchError(String),
    AppNotFound,
}

impl fmt::Display for AppProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppProcessError::AppWaitError => write!(f, "Failed to wait for application to close"),
            AppProcessError::AppLaunchError(msg) => {
                write!(f, "Failed to launch application: {}", msg)
            }
            AppProcessError::AppNotFound => write!(f, "Application not found"),
        }
    }
}

impl std::error::Error for AppProcessError {}
