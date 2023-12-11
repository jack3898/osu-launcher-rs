use std::{path::Path, process::Command};

use notify::{Event, EventKind, RecursiveMode, Watcher};
use tokio::task::JoinHandle;

use crate::{
    config::{data::DanserData, traits::app_data::Application},
    util::{general::unwrap_all_option, win::is_async_key_pressed},
};

pub fn try_spawn_danser_process(app: &DanserData) -> Option<JoinHandle<()>> {
    if !app.get_enabled() {
        return None;
    }

    let danser_options = unwrap_all_option(vec![
        app.get_executable_path(),
        app.settings_name.clone(),
        app.osu_replays_path.clone(),
    ]);

    if let Some(danser_options) = danser_options {
        let [danser_executable_path, danser_settings_name, replays_dir] =
            danser_options.try_into().unwrap();

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

            return Some(watcher_task);
        }
    }

    None
}
