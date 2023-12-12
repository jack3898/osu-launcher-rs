use std::{path::Path, process::Command};

use crate::{
    config::{
        data::{AppJoinHandle, DanserData},
        error::app_process_error::AppProcessError,
        traits::app_data::Application,
    },
    util::win::is_async_key_pressed,
};
use notify::{Event, EventKind, RecursiveMode, Watcher};

pub fn try_spawn_danser_process(app: &DanserData) -> Result<AppJoinHandle, AppProcessError> {
    if !app.get_enabled() {
        return Err(AppProcessError::AppLaunchError(
            "Danser is not enabled".to_string(),
        ));
    }

    let danser_executable_path = app.get_executable_path().unwrap();
    let danser_settings_name = app.settings_name.clone().unwrap();
    let replays_dir = app.get_replays_path().clone().unwrap();

    if app.executable_exists() {
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

        return Ok(watcher_task);
    }

    Err(AppProcessError::AppLaunchError(
        "Danser executable not found".to_string(),
    ))
}
