use std::fs;
use std::path::Path;

use crate::config::data::ConfigData;
use crate::util::file::file_exists;
use serde_json;

#[derive(Clone)]
pub struct LauncherConfig<'a> {
    pub file_path: &'a str,
    pub config: ConfigData,
}

impl<'a> From<&'a str> for LauncherConfig<'a> {
    fn from(file_path: &'a str) -> LauncherConfig<'a> {
        if LauncherConfig::config_file_exists(file_path) {
            let current_config_file_data = LauncherConfig::read_config_file(file_path);

            return match current_config_file_data {
                Ok(data) => LauncherConfig {
                    file_path: file_path,
                    config: data.clone(),
                },
                Err(message) => {
                    println!("{}", message);

                    return LauncherConfig {
                        file_path: file_path,
                        config: ConfigData::new(),
                    };
                }
            };
        }

        let config = LauncherConfig {
            file_path: file_path,
            config: ConfigData::new(),
        };

        config.create_config_file();

        return config;
    }
}

impl<'a> LauncherConfig<'a> {
    fn create_config_file(&self) {
        let config_data = serde_json::to_string_pretty(&self.config)
            .unwrap_or_else(|e| panic!("Error creating config data: {}", e));

        let data: &str = &config_data;
        let config_file_path = Path::new(self.file_path);

        fs::write(config_file_path, data)
            .expect(format!("Error creating config file: {}", self.file_path).as_str());
    }

    fn read_config_file(file_path: &'a str) -> Result<ConfigData, String> {
        let config_data_string = fs::read_to_string(file_path).expect("Error reading config file");
        let parsed_config: Result<ConfigData, serde_json::Error> =
            serde_json::from_str(&config_data_string);

        match parsed_config {
            Ok(data) => Ok(data),
            Err(error) => Err(format!("Error parsing config file: {}", error)),
        }
    }

    fn config_file_exists(file_path: &'a str) -> bool {
        file_exists(file_path)
    }
}
