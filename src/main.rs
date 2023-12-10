mod config;
mod util;

use config::data::AppData;
use config::manager::LauncherConfig;
use futures::future::join_all;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::{path::Path, process::Command};
use util::general::unwrap_all_option;
use util::win::is_async_key_pressed;

#[tokio::main]
async fn main() {
    // watcher is declared here because it needs to be in scope for the lifetime of the program
    let mut process_list = Vec::new();
    let launcher = LauncherConfig::from("./launcher_config.json");

    if launcher.config.danser.download && !launcher.config.danser.path_exists() {
        println!("Downloading Danser...");

        launcher.config.danser.download_application().await.unwrap();
    }

    if launcher.config.osu_trainer.download && !launcher.config.osu_trainer.path_exists() {
        println!("Downloading Osu! Trainer...");

        launcher
            .config
            .osu_trainer
            .download_application()
            .await
            .unwrap();
    }

    if launcher.config.open_tablet_driver.download
        && !launcher.config.open_tablet_driver.path_exists()
    {
        println!("Downloading OpenTabletDriver...");

        launcher
            .config
            .open_tablet_driver
            .download_application()
            .await
            .unwrap();
    }

    if let Some(osu_executable_path) = launcher.config.osu.get_executable_path() {
        if launcher.config.osu.executable_exists() {
            let child_future = tokio::spawn(async move {
                let mut osu_process = Command::new(osu_executable_path)
                    .spawn()
                    .expect("Failed to launch osu!");

                osu_process.wait().expect("Failed to wait for osu!");
            });

            process_list.push(child_future);
        }
    }

    if let Some(rewind_executable_path) = launcher.config.rewind.get_executable_path() {
        if launcher.config.rewind.executable_exists() {
            let child_future = tokio::spawn(async move {
                let mut rewind_process = Command::new(rewind_executable_path)
                    .spawn()
                    .expect("Failed to launch Rewind");

                rewind_process.wait().expect("Failed to wait for Rewind");
            });

            process_list.push(child_future);
        }
    }

    let danser_options = unwrap_all_option(vec![
        launcher.config.danser.get_executable_path(),
        launcher.config.danser.settings_name.clone(),
        launcher.config.osu.replays_dir.clone(),
    ]);

    if let Some(danser_options) = danser_options {
        let [danser_executable_path, danser_settings_name, replays_dir] =
            danser_options.try_into().unwrap();

        if launcher.config.danser.executable_exists() {
            let watcher_task = tokio::task::spawn_blocking(move || {
                let mut _watcher =
                    notify::recommended_watcher(move |res: Result<Event, _>| match res {
                        Ok(event) => match event.kind {
                            EventKind::Create(_) => {
                                let full_path = event.paths[0].to_str().unwrap();
                                let file_name =
                                    Path::new(&full_path).file_name().unwrap().to_str().unwrap();

                                // if R key is held at this moment
                                if is_async_key_pressed(0x52).unwrap_or(false) {
                                    println!("Rendering replay: {}", &file_name);

                                    Command::new(danser_executable_path.clone())
                                        .arg(format!("--out={}", file_name))
                                        .arg(format!("--settings={}", danser_settings_name))
                                        .arg(format!("--replay={}", full_path))
                                        .arg("--quickstart")
                                        .spawn()
                                        .expect("Failed to launch Danser");
                                }
                            }
                            _ => (),
                        },
                        Err(e) => panic!("Error watching directory: {}", e),
                    });

                let replays_dir = Path::new(&replays_dir);

                if let Ok(ref mut watcher_value) = _watcher {
                    match watcher_value.watch(replays_dir, RecursiveMode::Recursive) {
                        Ok(_) => println!("Watching directory: {:?}", replays_dir),
                        Err(e) => panic!("Error watching directory: {}", e),
                    }
                }

                loop {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });

            process_list.push(watcher_task);
        }
    }

    if let Some(open_tablet_driver_executable_path) =
        launcher.config.open_tablet_driver.get_executable_path()
    {
        if launcher.config.open_tablet_driver.executable_exists() {
            let child_future = tokio::spawn(async move {
                let mut open_tablet_driver_process =
                    Command::new(open_tablet_driver_executable_path)
                        .spawn()
                        .expect("Failed to launch OpenTabletDriver");

                open_tablet_driver_process
                    .wait()
                    .expect("Failed to wait for OpenTabletDriver");
            });

            process_list.push(child_future);
        }
    }

    if let Some(osu_trainer_executable_path) = launcher.config.osu_trainer.get_executable_path() {
        if launcher.config.osu_trainer.executable_exists() {
            let child_future = tokio::spawn(async move {
                let mut osu_trainer_process = Command::new(osu_trainer_executable_path)
                    .spawn()
                    .expect("Failed to launch osu!trainer");

                osu_trainer_process
                    .wait()
                    .expect("Failed to wait for osu!trainer");
            });

            process_list.push(child_future);
        }
    }

    join_all(process_list).await;
}
