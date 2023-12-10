use std::fs;
use std::path::Path;

use crate::config::data::ConfigData;
use crate::util::file::file_exists;
use serde_json;

pub struct LauncherConfig<'a> {
    pub file_path: &'a str,
    pub config: ConfigData,
}

impl<'a> LauncherConfig<'a> {
    pub fn from(file_path: &'a str) -> LauncherConfig<'a> {
        if LauncherConfig::config_file_exists(file_path) {
            println!("Config file exists, reading config file.");

            let current_config_file_data = LauncherConfig::read_config_file(file_path);

            return match current_config_file_data {
                Some(data) => LauncherConfig {
                    file_path: file_path,
                    config: data.clone(),
                },
                None => LauncherConfig {
                    file_path: file_path,
                    config: ConfigData::new(),
                },
            };
        } else {
            println!("Config file does not exist, creating config file with defaults.");

            let config = LauncherConfig {
                file_path: file_path,
                config: ConfigData::new(),
            };

            config.create_config_file();

            return config;
        }
    }

    fn create_config_file(&self) {
        println!("Creating config file");

        let config_data = serde_json::to_string_pretty(&self.config)
            .unwrap_or_else(|e| panic!("Error creating config data: {}", e));

        println!("Config data: {}", config_data);

        let data: &str = &config_data;
        let config_file_path = Path::new(self.file_path);

        match fs::write(config_file_path, data) {
            Ok(_) => println!("Successfully created config file"),
            Err(e) => panic!("Error creating config file: {}", e),
        };
    }

    fn read_config_file(file_path: &'a str) -> Option<ConfigData> {
        let config_data_string = fs::read_to_string(file_path);

        match config_data_string {
            Ok(data) => serde_json::from_str(&data).unwrap_or_else(|e| {
                println!("Error reading config file. Using defaults instead. {}", e);

                return None;
            }),
            Err(_) => None,
        }
    }

    fn config_file_exists(file_path: &'a str) -> bool {
        file_exists(file_path)
    }
}
