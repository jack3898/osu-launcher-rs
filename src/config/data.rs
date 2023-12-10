use serde::{Deserialize, Serialize};
use std::{env, path::Path};

use crate::util::win;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    pub osu_executable_path: Option<String>,
    pub rewind_executable_path: Option<String>,
    pub danser_executable_path: Option<String>,
    pub open_tablet_driver_executable_path: Option<String>,
    pub osu_trainer_executable_path: Option<String>,
    pub danser_out_dir: Option<String>,
    pub danser_settings_name: Option<String>,
    pub replays_dir: Option<String>,
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

        let danser_out_dir = match win::video_dir() {
            Some(video_dir) => Some(video_dir.to_string_lossy().into_owned()),
            None => None,
        };

        ConfigData {
            osu_executable_path: Some(osu_executable_path),
            rewind_executable_path: Some(rewind_executable_path),
            danser_executable_path: None,
            open_tablet_driver_executable_path: None,
            osu_trainer_executable_path: None,
            danser_out_dir: danser_out_dir,
            replays_dir: Some(replays_dir),
            danser_settings_name: Some(String::from("default")),
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
