mod config;
mod process;
mod util;

use std::io::stdin;

use config::manager::LauncherConfig;
use config::traits::app_data::AppData;
use futures::future::join_all;
use tokio::task::JoinHandle;
use util::file::file_exists;

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

    if launcher_config.config.osu_trainer.download
        && !launcher_config.config.osu_trainer.path_exists()
    {
        println!("Downloading Osu! Trainer...");

        launcher_config
            .config
            .osu_trainer
            .download_application()
            .await
            .unwrap();
    }

    if launcher_config.config.danser.download && !launcher_config.config.danser.path_exists() {
        println!("Downloading Danser...");

        launcher_config
            .config
            .danser
            .download_application()
            .await
            .unwrap();
    }

    if launcher_config.config.open_tablet_driver.download
        && !launcher_config.config.open_tablet_driver.path_exists()
    {
        println!("Downloading OpenTabletDriver...");

        launcher_config
            .config
            .open_tablet_driver
            .download_application()
            .await
            .unwrap();
    }

    let osu_process_handle = process::try_spawn_osu_process(launcher_config.clone());
    let rewind_process_handle = process::try_spawn_rewind_process(launcher_config.clone());
    let danser_process_handle = process::try_spawn_danser_process(launcher_config.clone());
    let open_tablet_driver_process_handle =
        process::try_spawn_open_tablet_driver_process(launcher_config.clone());
    let osu_trainer_process_handle =
        process::try_spawn_osu_trainer_process(launcher_config.clone());

    let mut process_list: Vec<JoinHandle<()>> = Vec::new();

    if osu_process_handle.is_some() {
        process_list.push(osu_process_handle.unwrap());
    }

    if rewind_process_handle.is_some() {
        process_list.push(rewind_process_handle.unwrap());
    }

    if danser_process_handle.is_some() {
        process_list.push(danser_process_handle.unwrap());
    }

    if open_tablet_driver_process_handle.is_some() {
        process_list.push(open_tablet_driver_process_handle.unwrap());
    }

    if osu_trainer_process_handle.is_some() {
        process_list.push(osu_trainer_process_handle.unwrap());
    }

    join_all(process_list).await;
}
