use serde::{Deserialize, Serialize};
use std::{env, path::Path};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    pub osu_executable_path: String,
    pub rewind_executable_path: String,
    pub danser_executable_path: String,
    pub open_tablet_driver_executable_path: String,
    pub osu_trainer_executable_path: String,
    pub danser_out_dir: String,
    pub osu_trainer_setting_name: String,
}

impl ConfigData {
    pub fn new() -> ConfigData {
        let app_data_location: String = ConfigData::get_app_data_location();

        ConfigData {
            osu_executable_path: Path::new(&app_data_location)
                .join("osu!")
                .join("osu!.exe")
                .to_string_lossy()
                .into_owned(),
            rewind_executable_path: Path::new(&app_data_location)
                .join("Programs")
                .join("Rewind")
                .join("Rewind.exe")
                .to_string_lossy()
                .into_owned(),
            danser_executable_path: String::from(""),
            open_tablet_driver_executable_path: String::from(""),
            osu_trainer_executable_path: String::from(""),
            danser_out_dir: Path::new(&app_data_location)
                .join("osu!")
                .join("Danser recordings")
                .to_string_lossy()
                .into_owned(),
            osu_trainer_setting_name: String::from("default"),
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
