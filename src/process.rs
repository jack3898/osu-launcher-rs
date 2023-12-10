use std::{path::Path, process::Command};

use notify::{Event, EventKind, RecursiveMode, Watcher};
use tokio::task::JoinHandle;

use crate::{
    config::{manager::LauncherConfig, traits::app_data::AppData},
    util::{general::unwrap_all_option, win::is_async_key_pressed},
};

pub fn try_spawn_osu_process(launcher_config: LauncherConfig) -> Option<JoinHandle<()>> {
    if !launcher_config.config.osu.enabled {
        return None;
    }

    if let Some(osu_executable_path) = launcher_config.config.osu.get_executable_path() {
        if launcher_config.config.osu.executable_exists() {
            let child_future = tokio::spawn(async move {
                let mut osu_process = Command::new(osu_executable_path)
                    .spawn()
                    .expect("Failed to launch osu!");

                osu_process.wait().expect("Failed to wait for osu!");
            });

            return Some(child_future);
        }
    }

    None
}

pub fn try_spawn_rewind_process(launcher_config: LauncherConfig) -> Option<JoinHandle<()>> {
    if !launcher_config.config.rewind.enabled {
        return None;
    }

    if let Some(rewind_executable_path) = launcher_config.config.rewind.get_executable_path() {
        if launcher_config.config.rewind.executable_exists() {
            let child_future = tokio::spawn(async move {
                let mut rewind_process = Command::new(rewind_executable_path)
                    .spawn()
                    .expect("Failed to launch Rewind");

                rewind_process.wait().expect("Failed to wait for Rewind");
            });

            return Some(child_future);
        }
    }

    None
}

pub fn try_spawn_danser_process(launcher_config: LauncherConfig) -> Option<JoinHandle<()>> {
    if !launcher_config.config.danser.enabled {
        return None;
    }

    let danser_options = unwrap_all_option(vec![
        launcher_config.config.danser.get_executable_path(),
        launcher_config.config.danser.settings_name.clone(),
        launcher_config.config.osu.replays_dir.clone(),
    ]);

    if let Some(danser_options) = danser_options {
        let [danser_executable_path, danser_settings_name, replays_dir] =
            danser_options.try_into().unwrap();

        if launcher_config.config.danser.executable_exists() {
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

            return Some(watcher_task);
        }
    }

    None
}

pub fn try_spawn_open_tablet_driver_process(
    launcher_config: LauncherConfig,
) -> Option<JoinHandle<()>> {
    if !launcher_config.config.open_tablet_driver.enabled {
        return None;
    }

    if let Some(open_tablet_driver_executable_path) = launcher_config
        .config
        .open_tablet_driver
        .get_executable_path()
    {
        if launcher_config
            .config
            .open_tablet_driver
            .executable_exists()
        {
            let child_future = tokio::spawn(async move {
                let mut open_tablet_driver_process =
                    Command::new(open_tablet_driver_executable_path)
                        .spawn()
                        .expect("Failed to launch OpenTabletDriver");

                open_tablet_driver_process
                    .wait()
                    .expect("Failed to wait for OpenTabletDriver");
            });

            return Some(child_future);
        }
    }

    None
}

pub fn try_spawn_osu_trainer_process(launcher_config: LauncherConfig) -> Option<JoinHandle<()>> {
    if !launcher_config.config.osu_trainer.enabled {
        return None;
    }

    if let Some(osu_trainer_executable_path) =
        launcher_config.config.osu_trainer.get_executable_path()
    {
        if launcher_config.config.osu_trainer.executable_exists() {
            let child_future = tokio::spawn(async move {
                let mut osu_trainer_process = Command::new(osu_trainer_executable_path)
                    .spawn()
                    .expect("Failed to launch osu!trainer");

                osu_trainer_process
                    .wait()
                    .expect("Failed to wait for osu!trainer");
            });

            return Some(child_future);
        }
    }

    None
}
