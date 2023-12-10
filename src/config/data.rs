use std::{env, path::Path};

use crate::config::traits::app_data::AppData;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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

        // Generate sensible defaults
        let osu_path = Path::new(&app_data_location)
            .join("osu!")
            .to_string_lossy()
            .into_owned();

        let rewind_path = Path::new(&app_data_location)
            .join("Programs")
            .join("Rewind")
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
            .to_string_lossy()
            .into_owned();

        let open_tablet_driver_path = Path::new(".")
            .join("packages")
            .join("opentabletdriver")
            .to_string_lossy()
            .into_owned();

        let osu_trainer_path = Path::new(".")
            .join("packages")
            .join("trainer")
            .to_string_lossy()
            .into_owned();

        // This is what will be written to the config file
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
                executable_name: Some(String::from("danser-cli.exe")),
                settings_name: Some(String::from("default")),
                download: true,
                enabled: false,
            },
            open_tablet_driver: OpenTabletDriverData {
                path: Some(open_tablet_driver_path),
                executable_name: Some(String::from("OpenTabletDriver.Daemon.exe")),
                download: true,
                enabled: false,
            },
            osu_trainer: OsuTrainerData {
                path: Some(osu_trainer_path),
                executable_name: Some(String::from("osu-trainer-v1.7.0/osu-trainer.exe")),
                download: true,
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
    pub settings_name: Option<String>,
    pub download: bool,
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
    pub download: bool,
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
    pub download: bool,
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
