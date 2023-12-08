use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    pub osu_path: String,
    pub rewind_path: String,
    pub danser_path: String,
    pub open_tablet_driver_path: String,
    pub osu_trainer_path: String,
}

impl ConfigData {
    pub fn new() -> ConfigData {
        ConfigData {
            osu_path: format!("{}\\osu!", ConfigData::get_app_data_location()),
            rewind_path: String::from(""),
            danser_path: String::from(""),
            open_tablet_driver_path: String::from(""),
            osu_trainer_path: String::from(""),
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
