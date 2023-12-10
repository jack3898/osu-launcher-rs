use serde::{Deserialize, Serialize};
use std::{env, path::Path};

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuData {
    pub executable_path: Option<String>,
    pub replays_dir: Option<String>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RewindData {
    pub executable_path: Option<String>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DanserData {
    pub executable_path: Option<String>,
    pub out_dir: Option<String>,
    pub settings_name: Option<String>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenTabletDriverData {
    pub executable_path: Option<String>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OsuTrainerData {
    pub executable_path: Option<String>,
    pub enabled: bool,
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
        let app_data_location: String = ConfigData::get_app_data_location();

        let osu_executable_path = Path::new(&app_data_location)
            .join("osu!")
            .join("osu!.exe")
            .to_string_lossy()
            .into_owned();

        let rewind_executable_path = Path::new(&app_data_location)
            .join("Programs")
            .join("Rewind")
            .join("Rewind.exe")
            .to_string_lossy()
            .into_owned();

        let replays_dir = Path::new(&app_data_location)
            .join("osu!")
            .join("Replays")
            .to_string_lossy()
            .into_owned();

        let danser_executable_path = Path::new(".")
            .join("packages")
            .join("danser")
            .join("danser.exe")
            .to_string_lossy()
            .into_owned();

        let open_tablet_driver_executable_path = Path::new(".")
            .join("packages")
            .join("opentabletdriver")
            .join("OpenTabletDriver.exe")
            .to_string_lossy()
            .into_owned();

        let osu_trainer_executable_path = Path::new(".")
            .join("packages")
            .join("trainer")
            .join("osu-trainer.exe")
            .to_string_lossy()
            .into_owned();

        ConfigData {
            osu: OsuData {
                executable_path: Some(osu_executable_path),
                replays_dir: Some(replays_dir),
                enabled: true,
            },
            rewind: RewindData {
                executable_path: Some(rewind_executable_path),
                enabled: true,
            },
            danser: DanserData {
                executable_path: Some(danser_executable_path),
                out_dir: None,
                settings_name: Some(String::from("default")),
                enabled: false,
            },
            open_tablet_driver: OpenTabletDriverData {
                executable_path: Some(open_tablet_driver_executable_path),
                enabled: false,
            },
            osu_trainer: OsuTrainerData {
                executable_path: Some(osu_trainer_executable_path),
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
