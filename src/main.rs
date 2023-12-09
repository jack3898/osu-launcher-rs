mod config;
mod util;

use config::manager::LauncherConfig;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    path::Path,
    process::{Child, Command},
};
use util::file::{file_exists, path_exists};

fn main() {
    // watcher is declared here because it needs to be in scope for the lifetime of the program
    let mut _watcher: Option<RecommendedWatcher> = None;
    let mut process_list: Vec<Child> = vec![];
    let launcher = LauncherConfig::from("./launcher_config.json");

    if file_exists(&launcher.config.osu_executable_path) {
        let osu_process = Command::new(launcher.config.osu_executable_path)
            .spawn()
            .expect("Failed to launch osu!");

        process_list.push(osu_process);
    }

    if file_exists(&launcher.config.rewind_executable_path) {
        let rewind_process = Command::new(launcher.config.rewind_executable_path)
            .spawn()
            .expect("Failed to launch Rewind");

        process_list.push(rewind_process);
    }

    if file_exists(&launcher.config.danser_executable_path)
        && path_exists(&launcher.config.danser_out_dir)
    {
        _watcher = Some(
            notify::recommended_watcher(move |res: Result<Event, _>| match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) => {
                        let full_path = event.paths[0].to_str().unwrap();
                        let file_name =
                            Path::new(&full_path).file_name().unwrap().to_str().unwrap();

                        println!("Rendering replay: {}", file_name);

                        Command::new(launcher.config.danser_executable_path.clone())
                            .arg(format!("--out={}", file_name))
                            .arg("--settings=default")
                            .arg(format!("--replay={}", full_path))
                            .spawn()
                            .expect("Failed to launch Danser");
                    }
                    _ => (),
                },
                Err(e) => panic!("Error watching directory: {}", e),
            })
            .unwrap(),
        );

        let replays_dir = Path::new(&launcher.config.replays_dir);

        if let Some(ref mut watcher_value) = _watcher {
            match watcher_value.watch(replays_dir, RecursiveMode::Recursive) {
                Ok(_) => println!("Watching directory: {:?}", replays_dir),
                Err(e) => panic!("Error watching directory: {}", e),
            }
        }
    }

    if file_exists(&launcher.config.open_tablet_driver_executable_path) {
        let open_tablet_driver_process =
            Command::new(launcher.config.open_tablet_driver_executable_path)
                .spawn()
                .expect("Failed to launch OpenTabletDriver");

        process_list.push(open_tablet_driver_process);
    }

    if file_exists(&launcher.config.osu_trainer_executable_path) {
        let osu_trainer_process = Command::new(launcher.config.osu_trainer_executable_path)
            .spawn()
            .expect("Failed to launch osu!trainer");

        process_list.push(osu_trainer_process);
    }

    for mut process in process_list {
        process.wait().expect("Failed to wait for process");
    }
}
