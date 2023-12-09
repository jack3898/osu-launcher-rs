mod config;
mod util;

use config::manager::LauncherConfig;
use std::process::{Child, Command};
use util::file::file_exists;

fn main() {
    let config = LauncherConfig::new("./launcher_config.json");

    let mut process_list: Vec<Child> = vec![];

    if file_exists(&config.config.osu_executable_path) {
        let osu_process = Command::new(config.config.osu_executable_path)
            .spawn()
            .expect("Failed to launch osu!");

        process_list.push(osu_process);
    }

    if file_exists(&config.config.rewind_executable_path) {
        let rewind_process = Command::new(config.config.rewind_executable_path)
            .spawn()
            .expect("Failed to launch Rewind");

        process_list.push(rewind_process);
    }

    if file_exists(&config.config.danser_executable_path) {
        let danser_process = Command::new(config.config.danser_executable_path)
            .arg("--record")
            .arg("--settings=\"Fast\"")
            .spawn()
            .expect("Failed to launch Danser");

        process_list.push(danser_process);
    }

    if file_exists(&config.config.open_tablet_driver_executable_path) {
        let open_tablet_driver_process =
            Command::new(config.config.open_tablet_driver_executable_path)
                .spawn()
                .expect("Failed to launch OpenTabletDriver");

        process_list.push(open_tablet_driver_process);
    }

    for mut process in process_list {
        process.wait().expect("Failed to wait for process");
    }
}
