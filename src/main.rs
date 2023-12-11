mod config;
mod process;
mod util;

use std::io::stdin;

use config::manager::LauncherConfig;
use config::traits::app_data::Application;
use futures::future::join_all;
use util::file::{delete_file, extract_zip, file_exists};

#[tokio::main]
async fn main() {
    let config_file_path = "./launcher_config.json";
    let first_launch = !file_exists(config_file_path);
    let launcher_config = LauncherConfig::from(config_file_path);

    if first_launch {
        println!("First launch, please configure the launcher using the config file that was just created then run the launcher again.");
        println!("Press enter to exit...");

        stdin().read_line(&mut String::new()).unwrap();

        return;
    }

    let mut download_futures = vec![];

    if launcher_config.config.osu_trainer.can_download() {
        println!("Downloading and extracting Osu! Trainer...");

        let download = launcher_config.config.osu_trainer.download();

        download_futures.push(download);
    }

    if launcher_config.config.danser.can_download() {
        println!("Downloading and extracting Danser...");

        let download = launcher_config.config.danser.download();

        download_futures.push(download);
    }

    if launcher_config.config.open_tablet_driver.can_download() {
        println!("Downloading and extracting OpenTabletDriver...");

        let download = launcher_config.config.open_tablet_driver.download();

        download_futures.push(download);
    }

    let download_path_results = join_all(download_futures).await;

    for download_path_result in download_path_results {
        match download_path_result {
            Ok(zip_location) => {
                extract_zip(&zip_location)
                    .expect(format!("Failed to extract {}", zip_location).as_str());

                delete_file(&zip_location)
                    .expect(format!("Failed to delete {}", zip_location).as_str());
            }
            Err(error) => {
                println!("Error downloading file: {}", error);
            }
        }
    }

    let process_list: Vec<_> = [
        &launcher_config.config.osu as &dyn Application,
        &launcher_config.config.rewind as &dyn Application,
        &launcher_config.config.danser as &dyn Application,
        &launcher_config.config.open_tablet_driver as &dyn Application,
        &launcher_config.config.osu_trainer as &dyn Application,
    ]
    .into_iter()
    .filter_map(|config| config.try_spawn_process())
    .collect();

    join_all(process_list).await;
}
