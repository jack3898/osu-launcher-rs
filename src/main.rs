mod config;

use config::manager::LauncherConfig;
use std::process::{Child, Command};

fn main() {
    let config = LauncherConfig::new("./launcher_config.json");

    let mut process_list: Vec<Child> = vec![];

    if config.config.osu_path != "" {
        let osu_process = Command::new(config.config.osu_path)
            .spawn()
            .expect("Failed to launch osu!");

        process_list.push(osu_process);
    }

    if config.config.rewind_path != "" {
        let rewind_process = Command::new(config.config.rewind_path)
            .spawn()
            .expect("Failed to launch Rewind");

        process_list.push(rewind_process);
    }

    if config.config.danser_path != "" {
        let danser_process = Command::new(config.config.danser_path)
            .arg("--record")
            .arg("--settings=\"default\"")
            .spawn()
            .expect("Failed to launch Danser");

        process_list.push(danser_process);
    }

    if config.config.open_tablet_driver_path != "" {
        let open_tablet_driver_process = Command::new(config.config.open_tablet_driver_path)
            .spawn()
            .expect("Failed to launch OpenTabletDriver");

        process_list.push(open_tablet_driver_process);
    }

    for mut process in process_list {
        process.wait().expect("Failed to wait for process");
    }
}
