mod config;

use config::manager::LauncherConfig;
use std::process::Command;

fn main() {
    let config = LauncherConfig::new("./launcher_config.json");

    let mut osu_process = Command::new(config.config.osu_path)
        .spawn()
        .expect("Failed to launch osu!");

    let mut rewind_process = Command::new(config.config.rewind_path)
        .spawn()
        .expect("Failed to launch Rewind");

    let mut open_tablet_driver_process = Command::new(config.config.open_tablet_driver_path)
        .spawn()
        .expect("Failed to launch OpenTabletDriver");

    let process_list = vec![
        &mut osu_process,
        &mut rewind_process,
        &mut open_tablet_driver_process,
    ];

    for process in process_list {
        process.wait().expect("Failed to wait for process");
    }
}
