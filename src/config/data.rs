use serde::{Deserialize, Serialize};
use std::{env, path::Path};

use crate::util::file::download_file_to;
use async_trait::async_trait;

#[async_trait]
pub trait AppData {
    fn get_path(&self) -> Option<String> {
        None
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

    async fn download_application(&self, file_name: &str) -> Result<(), String> {
        println!("Attempting to download {}...", file_name);

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

        match download {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error downloading: {}", e)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuData {
    pub path: Option<String>,
    pub executable_name: Option<String>,
    pub replays_dir: Option<String>,
    pub enabled: bool,
}

#[async_trait]
impl AppData for OsuData {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_executable_name(&self) -> Option<String> {
        self.executable_name.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RewindData {
    pub path: Option<String>,
    pub executable_name: Option<String>,
    pub enabled: bool,
}

#[async_trait]
impl AppData for RewindData {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_executable_name(&self) -> Option<String> {
        self.executable_name.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DanserData {
    pub path: Option<String>,
    pub executable_name: Option<String>,
    pub out_dir: Option<String>,
    pub settings_name: Option<String>,
    pub enabled: bool,
}

#[async_trait]
impl AppData for DanserData {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_executable_name(&self) -> Option<String> {
        self.executable_name.clone()
    }

    fn get_public_download_url(&self) -> Option<String> {
        Some(String::from(
            "https://github.com/Wieku/danser-go/releases/download/0.9.1/danser-0.9.1-win.zip",
        ))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenTabletDriverData {
    pub path: Option<String>,
    pub executable_name: Option<String>,
    pub enabled: bool,
}

#[async_trait]
impl AppData for OpenTabletDriverData {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_executable_name(&self) -> Option<String> {
        self.executable_name.clone()
    }

    fn get_public_download_url(&self) -> Option<String> {
        Some(String::from(
            "https://github.com/OpenTabletDriver/OpenTabletDriver/releases/download/v0.6.3.0/OpenTabletDriver.win-x64.zip",
        ))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTrainerData {
    pub path: Option<String>,
    pub executable_name: Option<String>,
    pub enabled: bool,
}

#[async_trait]
impl AppData for OsuTrainerData {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_executable_name(&self) -> Option<String> {
        self.executable_name.clone()
    }

    fn get_public_download_url(&self) -> Option<String> {
        Some(String::from(
            "https://github.com/FunOrange/osu-trainer/releases/download/1.7.0/osu-trainer-v1.7.0.zip",
        ))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    pub osu: OsuData,
    pub rewind: RewindData,
    pub danser: DanserData,
    pub open_tablet_driver: OpenTabletDriverData,
    pub osu_trainer: OsuTrainerData,
}

impl ConfigData {
    pub fn new() -> ConfigData {
        let app_data_location = ConfigData::get_app_data_location();

        let osu_path = Path::new(&app_data_location)
            .join("osu!")
            // .join("osu!.exe")
            .to_string_lossy()
            .into_owned();

        let rewind_path = Path::new(&app_data_location)
            .join("Programs")
            .join("Rewind")
            // .join("Rewind.exe")
            .to_string_lossy()
            .into_owned();

        let replays_path = Path::new(&app_data_location)
            .join("osu!")
            .join("Replays")
            .to_string_lossy()
            .into_owned();

        let danser_path = Path::new(".")
            .join("packages")
            .join("danser")
            // .join("danser.exe")
            .to_string_lossy()
            .into_owned();

        let open_tablet_driver_path = Path::new(".")
            .join("packages")
            .join("opentabletdriver")
            // .join("OpenTabletDriver.exe")
            .to_string_lossy()
            .into_owned();

        let osu_trainer_path = Path::new(".")
            .join("packages")
            .join("trainer")
            // .join("osu-trainer.exe")
            .to_string_lossy()
            .into_owned();

        ConfigData {
            osu: OsuData {
                path: Some(osu_path),
                executable_name: Some(String::from("osu!.exe")),
                replays_dir: Some(replays_path),
                enabled: true,
            },
            rewind: RewindData {
                path: Some(rewind_path),
                executable_name: Some(String::from("Rewind.exe")),
                enabled: true,
            },
            danser: DanserData {
                path: Some(danser_path),
                executable_name: Some(String::from("danser.exe")),
                out_dir: None,
                settings_name: Some(String::from("default")),
                enabled: false,
            },
            open_tablet_driver: OpenTabletDriverData {
                path: Some(open_tablet_driver_path),
                executable_name: Some(String::from("OpenTabletDriver.exe")),
                enabled: false,
            },
            osu_trainer: OsuTrainerData {
                path: Some(osu_trainer_path),
                executable_name: Some(String::from("osu-trainer.exe")),
                enabled: false,
            },
        }
    }

    pub fn get_app_data_location() -> String {
        let local_app_data = env::var("LOCALAPPDATA");

        match local_app_data {
            Ok(path) => path,
            Err(e) => {
                println!("Error getting local app data: {}", e);

                return String::from("");
            }
        }
    }
}
